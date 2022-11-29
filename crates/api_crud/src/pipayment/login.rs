use crate::pipayment::client::*;
use crate::PerformCrud;
use actix_web::web::Data;
use bcrypt::{hash, DEFAULT_COST};
use lemmy_api_common::{
  person::*,
  pipayment::*,
  sensitive::Sensitive,
  utils::{password_length_check},
};
use lemmy_apub::{
  generate_followers_url, generate_inbox_url, generate_local_apub_endpoint,
  generate_shared_inbox_url, EndpointType,
};
use lemmy_db_schema::{
  newtypes::{CommunityId, PiPaymentId, PersonId, PiUserId},
  schema::local_user::email_verified,
  source::{
    community::*,
    local_user::{LocalUser, LocalUserInsertForm},
    person::*,
    pipayment::*,
    site::*,
  },
  traits::{ApubActor, Crud, Followable},
  utils::naive_now,
};
use lemmy_db_views::structs::{LocalUserView, SiteView};
use lemmy_db_views_actor::structs::PersonViewSafe;

use lemmy_utils::{
  apub::generate_actor_keypair,
  claims::Claims,
  error::LemmyError,
  settings::SETTINGS,
  utils::{check_slurs, is_valid_actor_name},
  ConnectionId,
};
use lemmy_websocket::LemmyContext;
use sha2::{Digest, Sha256};
use uuid::Uuid;

#[async_trait::async_trait(?Send)]
impl PerformCrud for PiLogin {
  type Response = LoginResponse;

  async fn perform(
    &self,
    context: &Data<LemmyContext>,
    _websocket_id: Option<ConnectionId>,
  ) -> Result<LoginResponse, LemmyError> {
    // Call login from client after Pi.authenticate

    let data: &PiLogin = &self;
    let settings = SETTINGS.to_owned();
    // Make sure site has open registration
    // if let Ok(site) = blocking(context.pool(), move |conn| Site::read_local_site(conn)).await? {
    //   if !site.open_registration {
    //     return Err(LemmyError::from_message("registration_closed"));
    //   }
    // }
    let site_view = SiteView::read_local(context.pool()).await?;
    let local_site = site_view.local_site;

    if !local_site.open_registration {
      return Err(LemmyError::from_message("registration_closed"));
    }


    if !settings.pinetwork.pi_free_login {
      //return Err(LemmyError::from_message("registration_closed"));
    }
    // Hide Pi user name, not store pi_uid
    let mut _pi_username = data.ea.account.clone();
    let mut _pi_uid = data.ea.puid;
    let _pi_token = data.ea.token.clone();

    println!(
      "PiLogin is processing for {} {} {} ",
      _pi_uid.unwrap(),
      _pi_username.clone(),
      _pi_token.clone()
    );

    // First, valid user token
    let user_dto = match pi_me(context.client(), &_pi_token.clone()).await {
      Ok(dto) => {
        _pi_username = dto.username.clone();
        _pi_uid = Some(dto.uid.clone());
        Some(dto)
      }
      Err(_e) => {
        // Pi Server error
        let err_type = format!(
          "Pi Server Error: User not found: {}, error: {}",
          &data.ea.account,
          _e.to_string()
        );
        return Err(LemmyError::from_message(&err_type));
      }
    };

    let mut sha256 = Sha256::new();
    sha256.update(settings.pi_seed());
    sha256.update(_pi_username.clone());
    let _pi_alias: String = format!("{:X}", sha256.finalize());
    let _pi_alias2 = _pi_alias.clone();
    let _pi_alias3 = _pi_alias.clone();
    //let _pi_alias = data.pi_username.to_owned();

    let mut username = _pi_username.clone();
    let mut _new_user: Sensitive<String> = Sensitive::from(username.clone());
    let mut _new_password = Sensitive::from("".to_string()); //info.password.to_owned();

    let person_id: PersonId;
    let mut pi_exist = false;
    let mut result = true;
    let mut create_new = false;

    match &data.info {
      Some(info) => {
        create_new = true;
        _new_user = info.username_or_email.clone();
        _new_password = info.password.clone();
      }
      None => {}
    }

    // Check if there are admins. False if admins exist
    // let no_admins = blocking(context.pool(), move |conn| {
    //   PersonViewSafe::admins(conn).map(|a| a.is_empty())
    // })
    // .await??;
    let no_admins = PersonViewSafe::admins(context.pool()).await.map(|a| a.is_empty());

    if create_new {
      password_length_check(&_new_password)?;
      // // Make sure passwords match
      // if info.password != info.password_verify {
      //   return Err(LemmyError::from_message("passwords_dont_match"));
      // }

      // // If its not the admin, check the captcha
      // if !no_admins && Settings::get().captcha.enabled {
      //   let check = context
      //     .chat_server()
      //     .send(CheckCaptcha {
      //       uuid:
      //         info
      //         .captcha_uuid
      //         .to_owned()
      //         .unwrap_or_else(|| "".to_string()),
      //       answer:
      //         info
      //         .captcha_answer
      //         .to_owned()
      //         .unwrap_or_else(|| "".to_string()),
      //     })
      //     .await?;
      //   if !check {
      //     return Err(LemmyError::from_message("captcha_incorrect"));
      //   }
      // }
    }

    // Find user exist ?
    let pi_person = match Person::find_by_extra_name(context.pool(), &_pi_alias).await
    {
      Ok(c) => Some(c),
      Err(_e) => None,
    };

    let mut external_id = None;
    match pi_person {
      Some(pi) => {
        pi_exist = true;
        person_id = pi.id;
        username = pi.name.clone();
        external_id = pi.external_id;
      }
      None => {
        if !create_new {
          let err_type = format!("Hi {}, you must register before login.", &username);
          println!("{} {}", _pi_uid.unwrap(), err_type);
          return Err(LemmyError::from_message(&err_type));
        }
      }
    }

    if pi_exist {
      let local_user_id;
      let username2 = username.clone();
      let _local_user = match LocalUserView::read_from_name(context.pool(), &username2.clone()).await
      {
        Ok(lcu) => lcu.local_user,
        Err(_e) => {
          let err_type = format!(
            "PiLogin local user not found {} {} {}",
            _pi_username.clone(),
            username.clone(),
            _e.to_string()
          );
          println!("{} {}", _pi_uid.unwrap().clone(), err_type);
          return Err(LemmyError::from_error_message(_e, &err_type));

          //  return Ok(PiRegisterResponse {
          //   success: false,
          //   jwt: format!(""),
          //   extra: Some(format!("{}",err_type)),
          //   });
        }
      };

      local_user_id = _local_user.id.clone();

      //  let password_hash = hash(_new_password.clone(), DEFAULT_COST).expect("Couldn't hash password");
      if create_new {
        let updated_local_user = match LocalUser::update_password(context.pool(), local_user_id.clone(), &_new_password).await
        {
          Ok(lcu) => lcu,
          Err(_e) => {
            let err_type = format!(
              "PiLogin: Update local user password error {} {}",
              &username.clone(),
              _e.to_string()
            );
            return Err(LemmyError::from_message(&err_type));
          }
        };
      }
      // let _pi_uid_search = _pi_uid.clone();
      // let  _payment = match blocking(context.pool(), move |conn| {
      //   PiPayment::find_by_pi_uid(&context.pool(), &_pi_uid_search)
      // })
      // .await?
      // {
      //   Ok(c) => {
      //     Some(c)
      //   }
      //   Err(_e) => {
      //     let err_type = format!("Invalid pi user id {}", &_new_user.clone());
      //     println!("{} {}", _pi_uid.clone(), err_type);
      //     return Err(LemmyError::from_message(&err_type).into());
      //   },
      // };

      return Ok(LoginResponse {
        jwt: Some(
          Claims::jwt(
            //updated_local_user.id.0,
            local_user_id.0,
            &context.secret().jwt_secret,
            &context.settings().hostname,
          )?
          .into(),
        ),
        verify_email_sent: false,
        registration_created: false,
      });
    } // Pi exist

    // We have to create both a person, and local_user
    //if !create_new 
    {
      let err_type = format!(
        "Auto create new account for Pioneers is disabled {} {}",
        &_new_user.to_string().clone(),
        &_pi_uid.unwrap().clone()
      );
      println!("{}", err_type);
      //return LemmyError::from_error_message(e, &err_type)?;
      return Err(LemmyError::from_message(&err_type).into());
    }

    
  }
}
