use crate::PerformCrud;
use actix_web::web::Data;
use bcrypt::{hash, DEFAULT_COST};
use lemmy_api_common::{
  person::LoginResponse,
  utils::{blocking, honeypot_check, password_length_check},
  web3::*,
};
use lemmy_apub::{
  generate_inbox_url, generate_local_apub_endpoint, generate_shared_inbox_url, EndpointType,
};
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
  apub::generate_actor_keypair,
  claims::Claims,
  error::LemmyError,
  settings::SETTINGS,
  utils::{check_slurs, eth_verify, is_valid_actor_name},
  ConnectionId,
};
use lemmy_websocket::{
  messages::{CheckCaptcha, CheckToken},
  LemmyContext,
};

#[async_trait::async_trait(?Send)]
impl PerformCrud for Web3Register {
  type Response = LoginResponse;

  async fn perform(
    &self,
    context: &Data<LemmyContext>,
    _websocket_id: Option<ConnectionId>,
  ) -> Result<LoginResponse, LemmyError> {
    let settings = SETTINGS.to_owned();
    let data: &Web3Register = &self;

    // no email verification, or applications if the site is not setup yet
    let (mut email_verification, mut require_application) = (false, false);

    let mut _result = true;
    // Make sure site has open registration
    if let Ok(site) = blocking(context.pool(), move |conn| Site::read_local_site(conn)).await? {
      if !site.open_registration {
        return Err(LemmyError::from_message("registration_closed"));
      }
      email_verification = site.require_email_verification;
      require_application = site.require_application;
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
      return Err(LemmyError::from_message("registration_closed"));
    }

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
    let no_admins = blocking(context.pool(), move |conn| {
      PersonViewSafe::admins(conn).map(|a| a.is_empty())
    })
    .await??;

    // If its not the admin, check the captcha
    if !no_admins && settings.captcha.enabled {
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

    check_slurs(&data.info.username, &context.settings().slur_regex())?;

    let actor_keypair = generate_actor_keypair()?;
    if !is_valid_actor_name(
      &data.info.username,
      context.settings().actor_name_max_length,
    ) {
      println!(
        "Invalid username {} {}",
        _address.to_owned(),
        &data.info.username
      );
      return Err(LemmyError::from_message("register:invalid_username"));
      //let err_type = format!("Register: invalid username {}", &data.info.username);
    }

    let actor_id = generate_local_apub_endpoint(
      EndpointType::Person,
      &data.info.username,
      &settings.get_protocol_and_hostname(),
    )?;

    // Hide Pi user name, not store pi_uid
    //let mut sha256 = Sha256::new();
    //sha256.update(settings.pi_seed());
    //sha256.update(data.pi_username.to_owned());
    //let _pi_alias: String = format!("{:X}", sha256.finalize());
    let _alias = _address.clone();
    let _alias2 = _alias.clone();
    let _alias3 = _alias.clone();

    let _new_user = data.info.username.to_owned();
    let _new_user2 = data.info.username.to_owned();
    let _new_password = data.info.password.to_owned();

    let person_id: PersonId;
    let mut _exist = false;

    let person = match blocking(context.pool(), move |conn| {
      Person::find_by_name(&conn, &_new_user)
    })
    .await?
    {
      Ok(c) => Some(c),
      Err(_e) => None,
    };

    let other_person = match blocking(context.pool(), move |conn| {
      Person::find_by_web3_address(&conn, &_alias)
    })
    .await?
    {
      Ok(c) => Some(c),
      Err(_e) => None,
    };

    let mut change_password = false;
    match other_person {
      Some(op) => {
        person_id = op.id;
        _exist = true;
        match person {
          Some(other) => {
            if op.extra_user_id != other.extra_user_id {
              let err_type = format!(
                "Web3Register: User {} is exist and belong to other Web3 Account ",
                &data.info.username
              );
              println!("{} {} {}", data.address.clone(), err_type, &_alias2);
              _result = false;
              return Err(LemmyError::from_message(&err_type).into());
              // return Ok(PiRegisterResponse {
              //   success: false,
              //   jwt: format!(""),
              //   extra: Some(format!("{}",err_type)),
              //   });
            } else {
              // Same name and account: change password ???
              change_password = true;
            }
          }
          None => {
            change_password = true;
            // Not allow change username
            let err_type = format!("Web3Register: You already have user name {}, change password", op.name);
            println!("{} {} {}", data.address.clone(), err_type, &_alias2);
            _result = false;
          }
        };
      }
      None => {
        match person {
          Some(_other) => {
            let err_type = format!(
              "Web3Register: User {} is exist and belong to other user",
              &data.info.username
            );
            println!("{} {} {}", data.address.clone(), err_type, &_alias2);
            return Err(LemmyError::from_message(&err_type).into());
          }
          None => {
            // No account relate with web3_address/site_username, we must completed the registration and create new user
          }
        };
      }
    }

    // Person is exist, change his password
    if change_password {
      let _password_hash =
        hash(_new_password.clone(), DEFAULT_COST).expect("Couldn't hash password");

      let local_user_id;
      let _local_user = match blocking(context.pool(), move |conn| {
        LocalUserView::read_from_name(&conn, &_new_user2)
      })
      .await?
      {
        Ok(lcu) => lcu,
        Err(_e) => {
          let err_type = format!(
            "Web3Register: Update local user not found {} {} {}",
            &data.info.username,
            &data.address.clone(),
            _e.to_string()
          );
          return Err(LemmyError::from_message(&err_type).into());
        }
      };
      local_user_id = _local_user.local_user.id.clone();

      let updated_local_user = match blocking(context.pool(), move |conn| {
        LocalUser::update_password(&conn, local_user_id, &_new_password)
      })
      .await
      {
        Ok(chp) => chp,
        Err(_e) => {
          let err_type = format!(
            "Web3Register: Update local user password error {} {} {}",
            &data.info.username,
            &data.address.clone(),
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
        verify_email_sent: false,
        registration_created: _local_user.local_user.accepted_application,
      });
    }

    // We have to create both a person, and local_user
    // Register the new person
    let person_form = PersonForm {
      name: data.info.username.to_owned(),
      actor_id: Some(actor_id.clone()),
      private_key: Some(Some(actor_keypair.private_key)),
      public_key: Some(actor_keypair.public_key),
      inbox_url: Some(generate_inbox_url(&actor_id)?),
      shared_inbox_url: Some(Some(generate_shared_inbox_url(&actor_id)?)),
      admin: Some(no_admins),
      extra_user_id: Some(_alias2),
      ..PersonForm::default()
    };

    // insert the person
    let inserted_person = match blocking(context.pool(), move |conn| {
      Person::create(conn, &person_form)
    })
    .await?
    {
      Ok(p) => p,
      Err(_e) => {
        let err_type = format!(
          "Web3Register: user_already_exists: {} {}, exists{},  err:{}",
          &data.info.username,
          _alias3,
          _exist,
          _e.to_string()
        );
        return Err(LemmyError::from_message(&err_type).into());
      }
    };

    //let inserted_person = inserted_person1.unwrap();
    let new_id = inserted_person.id.clone();
    // Create the local user
    let local_user_form = LocalUserForm {
      person_id: Some(new_id.clone()),
      email: Some(data.info.email.as_deref().map(|s| s.to_owned())),
      password_encrypted: Some(data.info.password.to_string()),
      show_nsfw: Some(data.info.show_nsfw),
      email_verified: Some(false),
      ..LocalUserForm::default()
    };

    let inserted_local_user = match blocking(context.pool(), move |conn| {
      LocalUser::register(conn, &local_user_form)
    })
    .await?
    {
      Ok(lu) => lu,
      Err(_e) => {
        let err_type = if _e.to_string()
          == "duplicate key value violates unique constraint \"local_user_email_key\""
        {
          "Web3Register: email_already_exists"
        } else {
          "Web3Register: user_already_exists"
        };

        // If the local user creation errored, then delete that person
        blocking(context.pool(), move |conn| {
          Person::delete(&conn, inserted_person.id.clone())
        })
        .await??;

        return Err(LemmyError::from_message(err_type).into());
      }
    };

    // Return the jwt / LoginResponse
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
  }
}
