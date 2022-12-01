use crate::PerformCrud;
use actix_web::web::Data;
use lemmy_api_common::{
  person::*,
  sensitive::Sensitive,
  utils::{password_length_check},
  web3::*,
};
use lemmy_db_schema::{
  newtypes::PersonId,
  source::{
    local_user::{LocalUser, },
    person::*,
  },
};
use lemmy_db_views::structs::{LocalUserView, SiteView};
//use lemmy_db_views_actor::structs::PersonViewSafe;

use lemmy_utils::{
  claims::Claims,
  error::LemmyError,
  settings::SETTINGS,
  utils::{eth_verify, },
  ConnectionId,
};
use lemmy_websocket::{messages::CheckToken, LemmyContext};

#[async_trait::async_trait(?Send)]
impl PerformCrud for Web3Login {
  type Response = LoginResponse;

  async fn perform(
    &self,
    context: &Data<LemmyContext>,
    _websocket_id: Option<ConnectionId>,
  ) -> Result<LoginResponse, LemmyError> {

    let data: &Web3Login = &self;
    let settings = SETTINGS.to_owned();

    let site_view = SiteView::read_local(context.pool()).await?;
    let local_site = site_view.local_site;

    // if !local_site.open_registration {
    //   return Err(LemmyError::from_message("registration_closed"));
    // }

    //email_verification = local_site.require_email_verification;
    //require_application = local_site.require_application;

    let mut _address = data.account.clone();
    let mut _signature = data.signature.clone();
    let _token = data.token.clone();
    let _cli_time = data.epoch;

    let text = format!(
      "LOGIN:{};TOKEN:{};TIME:{}",
      _address.clone(),
      _token.clone(),
      _cli_time.clone()
    );
    println!(
      "Web3Login is processing for {} - {} {} {} ",
      text.clone(),
      _address.clone(),
      _token.clone(),
      data.signature.clone()
    );
    if !eth_verify(_address.clone(), text.clone(), _signature) {
      println!(
        "Web3Login is wrong signature for {} - {} {} {} ",
        text.clone(),
        _address.clone(),
        _token.clone(),
        data.signature.clone()
      );
      return Err(LemmyError::from_message("registration_closed"));
    }

    let _alias: String = _address.clone();

    let mut username = _address.clone();
    let mut _new_user: Sensitive<String> = Sensitive::from(username.clone());
    let mut _new_password = Sensitive::from("".to_string()); //info.password.to_owned();

    //let person_id: PersonId;
    let mut _exist = false;
    let mut result = true;
    let mut _change_password = false;

    match &data.info {
      Some(info) => {
        _change_password = true;
        _new_user = info.username_or_email.clone();
        _new_password = info.password.clone();
      }
      None => {
        let err_type = format!("Server Error: Web3 user not provided: {}", &data.account);
        return Err(LemmyError::from_message(&err_type));
      }
    }

    // Check if there are admins. False if admins exist
    // let no_admins = blocking(context.pool(), move |conn| {
    //   PersonViewSafe::admins(conn).map(|a| a.is_empty())
    // })
    // .await??;
    // let no_admins = PersonViewSafe::admins(context.pool()).await.map(|a| a.is_empty()).unwrap();
    // If its not the admin, check the token
    if local_site.site_setup {
      let check = context
        .chat_server()
        .send(CheckToken {
          uuid: data.token.clone(),
          answer: "".to_string(),
        })
        .await?;
      if !check {
        return Err(LemmyError::from_message("token_incorrect"));
      }
    }

    // Find user exist ?
    let exist_person = match Person::find_by_extra_name(context.pool(), &_alias).await
    {
      Ok(c) => {
        _exist = true;
        //person_id = c.id.clone();
        Some(c)
      },
      Err(_e) => {
        None
      },
    };

    // let mut external_id = None;
    // match exist_person {
    //   Some(person) => {
    //     _exist = true;
    //     person_id = person.id;
    //     username = person.name.clone();
    //     external_id = person.external_id;
    //   }
    //   None => {
    //       let err_type = format!("Hi {}, you must register before login.", &username);
    //       println!("{} {}", _address.clone(), err_type);
    //       return Err(LemmyError::from_message(&err_type));
    //   }
    // }

    if _exist && exist_person.is_some(){
      let person_id = exist_person.unwrap().id;
      let _local_user = match LocalUserView::read_person(context.pool(), person_id.clone()).await
      {
        Ok(lcu) => lcu.local_user,
        Err(_e) => {
          let err_type = format!(
            "Web3 local user not found {} {} {}",
            _address.clone(),
            username.clone(),
            _e.to_string()
          );
          println!("{} {}", _address.clone(), err_type);
          return Err(LemmyError::from_error_message(_e, &err_type));
        }
      };

      let local_user_id = _local_user.id.clone();

      if _change_password {
        password_length_check(&_new_password)?;
        let updated_local_user = match LocalUser::update_password(context.pool(), local_user_id.clone(), &_new_password).await
        {
          Ok(lcu) => lcu,
          Err(_e) => {
            let err_type = format!(
              "Update user password error {} {}",
              &username.clone(),
              _e.to_string()
            );
            return Err(LemmyError::from_message(&err_type));
          }
        };
      }

      return Ok(LoginResponse {
        jwt: Some(
          Claims::jwt(
            local_user_id.0,
            &context.secret().jwt_secret,
            &context.settings().hostname,
          )?
          .into(),
        ),
        verify_email_sent: _local_user.email_verified,
        registration_created: _local_user.accepted_application,
      });
    } // User exist

    // We have to create both a person, and local_user
    //if !create_new {
      let err_type = format!(
        "Auto create new account for web3 is disabled {} {}, please register first",
        &_new_user.to_string().clone(),
        &_address.clone()
      );
      println!("{}", err_type);
      //return LemmyError::from_error_message(e, &err_type)?;
      return Err(LemmyError::from_message(&err_type).into());

  }
}
