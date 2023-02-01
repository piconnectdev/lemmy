use crate::pipayment::client::*;
use crate::PerformCrud;
use actix_web::web::Data;
use bcrypt::{hash, DEFAULT_COST};
use lemmy_api_common::{
  person::*,
  pipayment::*,
  sensitive::Sensitive,
  utils::{password_length_check},
  utils::{
    generate_followers_url,
    generate_inbox_url,
    generate_local_apub_endpoint,
    generate_shared_inbox_url,
    get_local_user_view_from_jwt,
    is_admin,
    EndpointType,
  },
};
use lemmy_db_schema::{
  newtypes::{CommunityId, PiPaymentId, PersonId, PiUserId},
  schema::local_user::email_verified,
  source::{
    community::*,
    local_user::{LocalUser, LocalUserInsertForm},
    person::*,
    pipayment::*,
    site::*, local_site::RegistrationMode,
  },
  traits::{ApubActor, Crud, Followable},
  utils::naive_now,
};
use lemmy_db_views::structs::{LocalUserView, SiteView};
use lemmy_db_views_actor::structs::PersonViewSafe;

use lemmy_utils::{
  claims::Claims,
  error::LemmyError,
  utils::{check_slurs, is_valid_actor_name},
  ConnectionId,
};
use lemmy_api_common::{context::LemmyContext};
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
    let site_view = SiteView::read_local(context.pool()).await?;
    let local_site = site_view.local_site;

    if local_site.registration_mode == RegistrationMode::Closed {
      //return Err(LemmyError::from_message("registration_closed"));
    }

    if !context.settings().pinetwork.pi_free_login {
      //return Err(LemmyError::from_message("registration_closed"));
    }
    // Hide Pi user name, not store pi_uid
    let mut _pi_username = data.ea.account.clone();
    let mut _pi_uid = Some(PiUserId(data.ea.uuid.unwrap_or_default()));
    let _pi_token = data.ea.token.clone();

    println!(
      "PiLogin is processing for {} {} {} ",
      _pi_username.clone(),
      _pi_uid.unwrap(),
      _pi_token.clone()
    );

    // First, valid user token
    let user_dto = match pi_me(context, &_pi_token.clone()).await {
      Ok(dto) => {
        _pi_username = dto.username.clone();
        _pi_uid = Some(dto.uid.clone());
        Some(dto)
      }
      Err(_e) => {
        // Pi Server error
        let err_type = format!(
          "Pi Network Server Error: User not found: {}, error: {}",
          &data.ea.account,
          _e.to_string()
        );
        return Err(LemmyError::from_message(&err_type));
      }
    };

    let _pi_alias = _pi_username.clone();
    let mut username = _pi_username.clone();

    let mut _new_user: Sensitive<String> = Sensitive::from(_pi_alias.clone());
    let mut _new_password = Sensitive::from("".to_string()); //info.password.to_owned();

    let person_id: PersonId;
    let mut pi_exist = false;
    let mut _change_passwd = false;

    match &data.info {
      Some(info) => {
        _change_passwd = true;
        _new_user = info.username_or_email.clone();
        _new_password = info.password.clone();
      }
      None => {}
    }

    if _change_passwd {
      password_length_check(&_new_password)?;
      // // Make sure passwords match
      // if info.password != info.password_verify {
      //   return Err(LemmyError::from_message("passwords_dont_match"));
      // }

      // if local_site.site_setup && local_site.captcha_enabled {
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
      //     return Err(LemmyError::from_message("captcha_incorrect").into());
      //   }
      // }
    }

    // Find user exist ?
    let person = match Person::find_by_extra_name(context.pool(), &_pi_alias).await
    {
      Ok(c) => Some(c),
      Err(_e) => None,
    };

    let mut external_id = None;
    let mut external_name = None;
    match person {
      Some(p) => {
        pi_exist = true;
        person_id = p.id;
        username = p.name.clone();
        external_id = p.external_id;
        external_name = p.external_name;
      }
      None => {
        if !_change_passwd {
          let err_type = format!("Hi {}, you must register before login (use Pi Browser).", &username.clone());
          return Err(LemmyError::from_message(&err_type));
        }
      }
    }

    if pi_exist {
      let local_user_id;
      let _local_user = match LocalUserView::read_from_name(context.pool(), &username.clone()).await
      {
        Ok(lcu) => lcu.local_user,
        Err(_e) => {
          let err_type = format!(
            "PiLogin local user not found {} {} {}",
            _pi_username.clone(),
            username.clone(),
            _e.to_string()
          );
          return Err(LemmyError::from_error_message(_e, &err_type));

        }
      };

      local_user_id = _local_user.id.clone();

      if _change_passwd {
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
    } // User exist

    // We have to create both a person, and local_user
    //if !create_new 
    {
      let err_type = format!(
        "Auto create new account is disabled {} {}",
        &_new_user.to_string().clone(),
        &_pi_uid.unwrap().clone()
      );
      //return LemmyError::from_error_message(e, &err_type)?;
      return Err(LemmyError::from_message(&err_type).into());
    }
  }
}
