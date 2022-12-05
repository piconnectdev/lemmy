use crate::PerformCrud;
use actix_web::web::Data;
use lemmy_api_common::{context::LemmyContext};
use lemmy_api_common::{
  person::LoginResponse,
  pipayment::*,
  //utils::{honeypot_check, password_length_check, local_site_to_slur_regex},
};
// use lemmy_apub::{
//   generate_inbox_url, generate_local_apub_endpoint, generate_shared_inbox_url, EndpointType,
// };
// use lemmy_db_schema::{
//   newtypes::PersonId,
//   source::{
//     local_user::{LocalUser, LocalUserInsertForm},
//     person::*,
//     site::*,
//   },
//   traits::{ Crud},
//   utils::naive_now,
// };
use lemmy_db_views::structs::{LocalUserView, SiteView};
use lemmy_db_views_actor::{ structs::PersonViewSafe};

use lemmy_utils::{
  error::LemmyError,
  settings::SETTINGS,
  ConnectionId,
};

use crate::web3::ext::*;

#[async_trait::async_trait(?Send)]
impl PerformCrud for PiRegister {
  type Response = LoginResponse;

  async fn perform(
    &self,
    context: &Data<LemmyContext>,
    _websocket_id: Option<ConnectionId>,
  ) -> Result<LoginResponse, LemmyError> {
    let settings = SETTINGS.to_owned();
    let data: &PiRegister = &self;
    let ext_account = data.ea.clone();
    // no email verification, or applications if the site is not setup yet
    //let (mut email_verification, mut require_application) = (false, false);

    //let mut result = true;

    let site_view = SiteView::read_local(context.pool()).await?;
    let local_site = site_view.local_site;

    if !local_site.open_registration {
      return Err(LemmyError::from_message("registration_closed"));
    }

    if local_site.site_setup {
      if !settings.pi_enabled {
        return Err(LemmyError::from_message("registration_disabled"));
      }
      if !settings.pinetwork.pi_allow_all {
        return Err(LemmyError::from_message("registration_disabled"));
      }
    }

    let login_response = match create_external_account(context, &ext_account.account.clone(), &ext_account, &data.info).await
    {
      Ok(c) => c,
      Err(_e) => {
        return Err(LemmyError::from_message("registration_disabled"));
      },
    };
    
    /*
    email_verification = local_site.require_email_verification;
    require_application = local_site.require_application;
    
    password_length_check(&data.info.password)?;
    honeypot_check(&data.info.honeypot)?;

    if email_verification && data.info.email.is_none() {
      return Err(LemmyError::from_message("email_required"));
    }

    if require_application && data.info.answer.is_none() {
      return Err(LemmyError::from_message(
        "registration_application_answer_required",
      ));
    }

    // Make sure passwords match
    if data.info.password != data.info.password_verify {
      return Err(LemmyError::from_message("passwords_dont_match"));
    }

    // Check if there are admins. False if admins exist
    // let no_admins = blocking(context.pool(), move |conn| {
    //   PersonViewSafe::admins(conn).map(|a| a.is_empty())
    // })
    // .await??;
    let no_admins = PersonViewSafe::admins(context.pool()).await.map(|a| a.is_empty());
    // If its not the admin, check the captcha
    //if !no_admins && settings.captcha.enabled {
    if local_site.site_setup && local_site.captcha_enabled {
      let check = context
        .chat_server()
        .send(CheckCaptcha {
          uuid: data
            .info
            .captcha_uuid
            .to_owned()
            .unwrap_or_else(|| "".to_string()),
          answer: data
            .info
            .captcha_answer
            .to_owned()
            .unwrap_or_else(|| "".to_string()),
        })
        .await?;
      if !check {
        return Err(LemmyError::from_message("captcha_incorrect").into());
      }
    }

    let slur_regex = local_site_to_slur_regex(&local_site);
    check_slurs(&data.info.username, &slur_regex)?;

    let actor_keypair = generate_actor_keypair()?;
    if !is_valid_actor_name(&data.info.username, local_site.actor_name_max_length as usize) {
      return Err(LemmyError::from_message("invalid_username"));
    }

    let actor_id = generate_local_apub_endpoint(
      EndpointType::Person,
      &data.info.username,
      &settings.get_protocol_and_hostname(),
    )?;

    // Hide Pi user name, not store pi_uid
    let mut sha256 = Sha256::new();
    sha256.update(settings.pi_seed());
    sha256.update(ext_account.account.to_owned());
    let _alias: String = format!("{:X}", sha256.finalize());
    
    //let _pi_uid = data.pi_uid.clone();
    let _new_user = data.info.username.to_owned();
    let _new_password = data.info.password.to_owned();

    let person_id: PersonId;
    let mut pi_exist = false;

    let person = match Person::find_by_name(context.pool(), &_new_user).await
    {
      Ok(c) => Some(c),
      Err(_e) => None,
    };

    let other_person = match Person::find_by_extra_name(context.pool(), &_alias).await
    {
      Ok(c) => Some(c),
      Err(_e) => None,
    };

    let mut change_password = false;
    match pi_person {
      Some(pi) => {
        person_id = pi.id;
        pi_exist = true;
        match person {
          Some(other) => {
            if pi.external_id != other.external_id {
              let err_type = format!(
                "PiRegister: User {} is exist and belong to other Pi Account ",
                &data.info.username
              );
              println!("{} {} {}", data.pi_username.clone(), err_type, &_pi_alias2);
              result = false;
              return Err(LemmyError::from_message(&err_type).into());
            } else {
              // Same name and account: change password ???
              change_password = true;
            }
          }
          None => {
            change_password = true;
            // Not allow change username
            let err_type = format!("PiRegister: You already have user name {}", pi.name);
            println!("{} {} {}", data.pi_username.clone(), err_type, &_pi_alias2);
            result = false;
            //return Err(LemmyError::from_message(&err_type).into());
            // return Ok(PiRegisterResponse {
            //   success: false,
            //   jwt: format!(""),
            //   extra: Some(format!("{}",err_type)),
            //   });
          }
        };
      }
      None => {
        match person {
          Some(other) => {
            let err_type = format!(
              "PiRegister: User {} is exist and belong to other user",
              &data.info.username
            );
            println!("{} {} {}", data.pi_username.clone(), err_type, &_pi_alias2);
            result = false;
            return Err(LemmyError::from_message(&err_type).into());
          }
          None => {
            // No account relate with pi_username/site_username, we must completed the payment and create new user
          }
        };
      }
    }

    // Person is exist, change his password
    if change_password {
      let password_hash =
        hash(_new_password.clone(), DEFAULT_COST).expect("Couldn't hash password");

      let local_user_id;
      let _local_user = match LocalUserView::read_from_name(context.pool(), &_new_user2)
      .await
      {
        Ok(lcu) => lcu,
        Err(_e) => {
          let err_type = format!(
            "PiRegister: Update local user not found {} {}",
            &data.info.username,
            _e.to_string()
          );
          return Err(LemmyError::from_message(&err_type).into());
        }
      };
      local_user_id = _local_user.local_user.id.clone();

      let updated_local_user = match LocalUser::update_password(context.pool(), local_user_id, &_new_password).await
      {
        Ok(chp) => chp,
        Err(_e) => {
          let err_type = format!(
            "PiRegister: Update local user password error {} {}",
            &data.info.username,
            _e.to_string()
          );
          return Err(LemmyError::from_message(&err_type).into());
        }
      };
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
        registration_created: false,
      });
    }

    let person_form = PersonInsertForm::builder()
      .name(data.info.username.clone())
      .actor_id(Some(actor_id.clone()))
      .private_key(Some(actor_keypair.private_key))
      .public_key(actor_keypair.public_key)
      .inbox_url(Some(generate_inbox_url(&actor_id)?))
      .shared_inbox_url(Some(generate_shared_inbox_url(&actor_id)?))
      // If its the initial site setup, they are an admin
      .admin(Some(!local_site.site_setup))
      .instance_id(site_view.site.instance_id)
      .external_id(Some(ext_account.account.to_owned()))
      .build();

    // insert the person
    // let err_type = format!("user_already_exists: {} {}", &data.info.username, _pi_alias3);
    let inserted_person = match Person::create(context.pool(), &person_form).await
    {
      Ok(p) => p,
      Err(_e) => {
        let err_type = format!(
          "Register: user_already_exists: {} {}, exists{},  err:{}",
          &data.info.username,
          _pi_alias3,
          pi_exist,
          _e.to_string()
        );
        return Err(LemmyError::from_message(&err_type).into());
      }
    };

    //let new_id = inserted_person.id.clone();

    let local_user_form = LocalUserInsertForm::builder()
      .person_id(inserted_person.id)
      .email(data.info.email.as_deref().map(str::to_lowercase))
      .password_encrypted(data.info.password.to_string())
      .show_nsfw(Some(data.info.show_nsfw))
      .email_verified(Some(false))
      .build();

    let inserted_local_user = match LocalUser::create(context.pool(), &local_user_form).await
    {
      Ok(lu) => lu,
      Err(_e) => {
        let err_type = if _e.to_string()
          == "duplicate key value violates unique constraint \"local_user_email_key\""
        {
          "Register: email_already_exists"
        } else {
          "Register: user_already_exists"
        };

        // If the local user creation errored, then delete that person
        Person::delete(context.pool(), inserted_person.id).await?;

        return Err(LemmyError::from_message(err_type).into());
      }
    };

    // Return the jwt / LoginResponse?
    Ok(LoginResponse {
      jwt: Some(
        Claims::jwt(
          inserted_local_user.id.0,
          &context.secret().jwt_secret,
          &context.settings().hostname,
        )?
        .into(),
      ),
      verify_email_sent: false,
      registration_created: false,
    })
     */
    Ok(login_response)
  }
}
