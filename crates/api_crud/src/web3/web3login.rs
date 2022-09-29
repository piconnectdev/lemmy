use crate::PerformCrud;
use actix_web::web::Data;
use bcrypt::{hash, DEFAULT_COST};
use lemmy_api_common::{
  person::*,
  sensitive::Sensitive,
  utils::{blocking, password_length_check},
  web3::*,
};
use lemmy_apub::{ EndpointType,};
use lemmy_db_schema::{
  newtypes::PersonId,
  source::{
    local_user::{LocalUser, LocalUserForm},
    person::*,
    site::*,
  },
  traits::{Crud},
};
use lemmy_db_views::structs::LocalUserView;
use lemmy_db_views_actor::structs::PersonViewSafe;

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
    // Make sure site has open registration
    if let Ok(site) = blocking(context.pool(), move |conn| Site::read_local_site(conn)).await? {
      if !site.open_registration {
        return Err(LemmyError::from_message("registration_closed"));
      }
    }

    let mut _address = data.address.clone();
    let mut _signature = data.signature.clone();
    let _token = data.token.clone();
    let _cli_time = data.cli_time;

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
    // TODO: First, valid user address
    // Check user if exists on blockchain
    /*
     let user_dto = match web3_exist(context.client(), &data.pi_token.clone()).await {
       Ok(dto) => {
         _pi_username = dto.username.clone();
         _pi_uid = dto.uid.clone();
         Some(dto)
       }
       Err(_e) => {
         // Pi Server error
         let err_type = format!("Pi Server Error: User not found: {}, error: {}", &data.pi_username,  _e.to_string());
         return Err(LemmyError::from_message(&err_type));
       }
     };
    */
    let _alias: String = _address.clone();
    let _alias2 = _alias.clone();
    let _alias3 = _alias.clone();

    let mut username = _address.clone();
    let mut _new_user: Sensitive<String> = Sensitive::from(username.clone());
    let mut _new_password = Sensitive::from("".to_string()); //info.password.to_owned();

    let person_id: PersonId;
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
        let err_type = format!("Server Error: Web3 user not provided: {}", &data.address);
        return Err(LemmyError::from_message(&err_type));
      }
    }

    // Check if there are admins. False if admins exist
    let no_admins = blocking(context.pool(), move |conn| {
      PersonViewSafe::admins(conn).map(|a| a.is_empty())
    })
    .await??;

    // If its not the admin, check the token
    if !no_admins {
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

    if _change_password {
      password_length_check(&_new_password)?;
    }

    // Find user exist ?
    let exist_person = match blocking(context.pool(), move |conn| {
      Person::find_by_web3_address(conn, &_alias)
    })
    .await?
    {
      Ok(c) => Some(c),
      Err(_e) => None,
    };

    let mut extra_user_id = None;
    match exist_person {
      Some(person) => {
        _exist = true;
        // person_id = person.id;
        username = person.name.clone();
        extra_user_id = person.extra_user_id;
      }
      None => {
          let err_type = format!("Hi {}, you must register before login.", &username);
          println!("{} {}", _address.clone(), err_type);
          return Err(LemmyError::from_message(&err_type));
      }
    }

    if _exist {
      let local_user_id;
      let username2 = username.clone();
      let _local_user = match blocking(context.pool(), move |conn| {
        LocalUserView::read_from_name(conn, &username2.clone())
      })
      .await?
      {
        Ok(lcu) => lcu,
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

      local_user_id = _local_user.local_user.id.clone();

      if _change_password {
        let updated_local_user = match blocking(context.pool(), move |conn| {
          LocalUser::update_password(conn, local_user_id.clone(), &_new_password)
        })
        .await
        {
          Ok(lcu) => lcu,
          Err(_e) => {
            let err_type = format!(
              "Web3: Update user password error {} {}",
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
        verify_email_sent: _local_user.local_user.email_verified,
        registration_created: _local_user.local_user.accepted_application,
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
