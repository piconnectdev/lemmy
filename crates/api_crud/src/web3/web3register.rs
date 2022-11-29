use crate::PerformCrud;
use actix_web::web::Data;
use lemmy_api_common::{
  person::LoginResponse,
  web3::*,
};
use lemmy_db_schema::{ *
  //newtypes::PersonId,
  //source::{
    //local_user::{LocalUser, LocalUserInsertForm},
    //person::*, registration_application::RegistrationApplicationInsertForm,
  //},
  //traits::{Crud}, aggregates::structs::PersonAggregates,
};
use lemmy_db_views::structs::{SiteView};
use lemmy_db_views_actor::structs::PersonViewSafe;

use lemmy_utils::{
  claims::Claims,
  error::LemmyError,
  settings::SETTINGS,
  utils::{eth_verify, },
  ConnectionId,
};
use lemmy_websocket::{
  messages::{CheckToken},
  LemmyContext,
};

use crate::web3::ext::*;

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
    let ext_account = data.external_account.clone();

    // no email verification, or applications if the site is not setup yet
    let (mut email_verification, mut require_application) = (false, false);

    let mut _result = true;

    let site_view = SiteView::read_local(context.pool()).await?;
    let local_site = site_view.local_site;

    if !local_site.open_registration {
      return Err(LemmyError::from_message("registration_closed"));
    }
    
    if local_site.site_setup {
      if !settings.web3_enabled {
        return Err(LemmyError::from_message("registration_disabled"));
      }
    }

    // If its not the admin, check the token / sign
    if local_site.site_setup {
      let mut _address = ext_account.account.clone();
      let mut _signature = ext_account.signature.clone();
      let _token = ext_account.token.clone();
      let _cli_time = ext_account.cli_time;
  
      let check = context
        .chat_server()
        .send(CheckToken {
          uuid: _token.clone(),
          answer: "".to_string(),
        })
        .await?;
      if !check {
        return Err(LemmyError::from_message("token_incorrect"));
      }

      let text = format!(
        "LOGIN:{};TOKEN:{};TIME:{}",
        _address.clone(),
        _token.clone(),
        _cli_time.clone()
      );
      println!(
        "Web3Registration is processing for {} - {} {} {} ",
        text.clone(),
        _address.clone(),
        _token.clone(),
        ext_account.signature.clone()
      );
      if !eth_verify(_address.clone(), text.clone(), _signature) {
        return Err(LemmyError::from_message("registration_closed"));
      }  
    }

    let login_response = match create_external_account(context, &ext_account.account.clone(), &ext_account, &data.info.clone()).await
    {
      Ok(c) => c,
      Err(_e) => {
        return Err(_e);
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
    //let no_admins = PersonViewSafe::admins(conn).map(|a| a.is_empty()).await;
    // let no_admins = PersonViewSafe::admins(context.pool()).await.map(|a| a.is_empty()).unwrap();
    // If its not the admin, check the captcha
    // If the site is set up, check the captcha
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
    //check_slurs_opt(&data.info.answer, &slur_regex)?;

    let actor_keypair = generate_actor_keypair()?;
    if !is_valid_actor_name(&data.info.username, local_site.actor_name_max_length as usize) {
      println!(
        "Invalid username {} {}",
        data.account.to_owned(),
        &data.info.username
      );

      return Err(LemmyError::from_message("register:invalid_username"));
    }

    let actor_id = generate_local_apub_endpoint(
      EndpointType::Person,
      &data.info.username,
      &settings.get_protocol_and_hostname(),
    )?;

    let _alias = ext_account.account.clone();
    let _new_user = data.info.username.to_owned();
    let _new_password = data.info.password.to_owned();

    let person_id: PersonId;
    let mut _exist = false;

    let person = match Person::find_by_name(context.pool(), &_new_user.clone()).await
    {
      Ok(c) => Some(c),
      Err(_e) => None,
    };

    let other_person = match Person::find_by_extra_name(context.pool(), &_alias.clone()).await
    {
      Ok(c) => Some(c),
      Err(_e) => None,
    };

    let mut change_password = false;
    match other_person {
      Some(op) => {
        person_id = op.id;
        _exist = true;
        change_password = true;
        match person {
          Some(other) => {
            if op.external_id != other.external_id {
              let err_type = format!(
                "Web3Register: User {} is exist and belong to other Web3 Account ",
                &data.info.username
              );
              println!("{} {} {}", data.account.clone(), err_type, &_alias.clone());
              _result = false;
              return Err(LemmyError::from_message(&err_type).into());
            } else {
              // Same name and account: change password ???
              change_password = true;
            }
          }
          None => {
            change_password = true;
            // Not allow change username
            let err_type = format!("Web3Register: You already have user name {}, change password", op.name);
            println!("{} {} {}", data.account.clone(), err_type, &_alias.clone());
            _result = false;
          }
        };
      }
      None => {
        match person {
          Some(_other) => {
            let err_type = format!(
              "User {} is exist and belong to other user",
              &data.info.username.clone()
            );
            println!("{} {} {}", _alias.clone(), err_type, &data.info.username.clone());
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

      //let _new_user = other_person.clone().unwrap().name.clone();
      //let _local_user = match LocalUserView::read_from_name(context.pool(), &_new_user.clone()).await
      let _local_user = match LocalUserView::read_person(context.pool(), person_id.clone()).await
      {
        Ok(lcu) => lcu,
        Err(_e) => {
          let err_type = format!(
            "Web3Register: Update local user not found {} {} {}",
            &data.info.username,
            &data.account.clone(),
            _e.to_string()
          );
          return Err(LemmyError::from_message(&err_type).into());
        }
      };

      let local_user_id = _local_user.local_user.id.clone();

      let updated_local_user = match LocalUser::update_password(context.pool(), local_user_id, &_new_password).await
      {
        Ok(chp) => chp,
        Err(_e) => {
          let err_type = format!(
            "Web3Register: Update local user password error {} {} {}",
            &data.info.username,
            &data.account.clone(),
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
        registration_created: _local_user.local_user.accepted_application,
      });
    }

    // We have to create both a person, and local_user
    // Register the new person
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
      .external_id(Some(_alias.clone()))
      .build();


    let inserted_person = match Person::create(context.pool(), &person_form).await
    {
      Ok(p) => p,
      Err(_e) => {
        let err_type = format!(
          "Web3Register: user_already_exists: {} {}, exists{},  err:{}",
          &data.info.username,
          _alias.clone(),
          _exist,
          _e.to_string()
        );
        return Err(LemmyError::from_message(&err_type).into());
      }
    };
    // Create the local user
    let local_user_form = LocalUserInsertForm::builder()
      .person_id(inserted_person.id)
      .email(data.info.email.as_deref().map(str::to_lowercase))
      .password_encrypted(data.info.password.to_string())
      .show_nsfw(Some(data.info.show_nsfw))
      .build();

     let inserted_local_user = match LocalUser::create(context.pool(), &local_user_form).await {
      Ok(lu) => lu,
      Err(e) => {
        let err_type = if e.to_string()
          == "duplicate key value violates unique constraint \"local_user_email_key\""
        {
          "email_already_exists"
        } else {
          "user_already_exists"
        };

        // If the local user creation errored, then delete that person
        Person::delete(context.pool(), inserted_person.id).await?;

        return Err(LemmyError::from_error_message(e, err_type));
      }
    };

    if local_site.site_setup && local_site.require_application {
      // Create the registration application
      let form = RegistrationApplicationInsertForm {
        local_user_id: inserted_local_user.id,
        // We already made sure answer was not null above
        answer: data.info.answer.clone().expect("must have an answer"),
      };

      RegistrationApplication::create(context.pool(), &form).await?;
    }

    // Email the admins
    if local_site.application_email_admins {
      send_new_applicant_email_to_admins(&data.username, context.pool(), context.settings())
        .await?;
    }

    let mut login_response = LoginResponse {
      jwt: None,
      registration_created: false,
      verify_email_sent: false,
    };

    // Log the user in directly if the site is not setup, or email verification and application aren't required
    if !local_site.site_setup
      || (!local_site.require_application && !local_site.require_email_verification)
    {
      login_response.jwt = Some(
        Claims::jwt(
          inserted_local_user.id.0,
          &context.secret().jwt_secret,
          &context.settings().hostname,
        )?
        .into(),
      );
    } else {
      if local_site.require_email_verification {
        let local_user_view = LocalUserView {
          local_user: inserted_local_user,
          person: inserted_person,
          counts: PersonAggregates::default(),
        };
        // we check at the beginning of this method that email is set
        let email = local_user_view
          .local_user
          .email
          .clone()
          .expect("email was provided");

        send_verification_email(&local_user_view, &email, context.pool(), context.settings())
          .await?;
        login_response.verify_email_sent = true;
      }

      if local_site.require_application {
        login_response.registration_created = true;
      }
    }
    */
    Ok(login_response)
  }
}

