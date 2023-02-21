use actix_web::web::Data;
use bcrypt::{hash, DEFAULT_COST};
use lemmy_api_common::{
  person::{LoginResponse, Register},
  utils::{honeypot_check, password_length_check, local_site_to_slur_regex, send_new_applicant_email_to_admins,send_verification_email},
  utils::{
    generate_inbox_url,
    generate_local_apub_endpoint,
    generate_shared_inbox_url,
    EndpointType,
  },
  context::LemmyContext,  
  web3::*,
};

use lemmy_db_schema::{
  newtypes::PersonId,
  source::{
    local_site::{LocalSite, RegistrationMode},
    local_user::{LocalUser, LocalUserInsertForm},
    person::*, registration_application::RegistrationApplicationInsertForm,
    registration_application::RegistrationApplication, community::Community, person_balance::{PersonBalanceInsertForm, PersonBalance}
  },
  traits::{Crud, ApubActor}, aggregates::structs::PersonAggregates,
};
use lemmy_db_views::structs::{LocalUserView, SiteView};

use lemmy_utils::{
  apub::generate_actor_keypair,
  claims::Claims,
  error::LemmyError,
  ConnectionId, utils::{web3::eth_verify, slurs::check_slurs, validation::is_valid_actor_name},
};


pub async fn create_external_account(context: &Data<LemmyContext>, ext_name: &str, ea: &ExternalAccount, info: &Register, kyced: bool) -> Result<LoginResponse, LemmyError> 
{
  let site_view = SiteView::read_local(context.pool()).await?;
  let local_site = site_view.local_site;

  // no email verification, or applications if the site is not setup yet
  let (mut email_verification, mut require_application) = (false, false);

  email_verification = local_site.require_email_verification;
  require_application = local_site.registration_mode == RegistrationMode::RequireApplication;

  password_length_check(&info.password)?;
  honeypot_check(&info.honeypot)?;

  if email_verification && info.email.is_none() {
    return Err(LemmyError::from_message("email_required"));
  }

  if require_application && info.answer.is_none() {
    return Err(LemmyError::from_message(
      "registration_application_answer_required",
    ));
  }

  // Make sure passwords match
  if info.password != info.password_verify {
    return Err(LemmyError::from_message("passwords_dont_match"));
  }

  if local_site.site_setup && local_site.captcha_enabled {
    let check = context
      .chat_server().check_captcha(
        info.captcha_uuid.to_owned().unwrap_or_else(|| "".to_string()),
        info.captcha_answer.to_owned().unwrap_or_else(|| "".to_string())
      )?;
    if !check {
      return Err(LemmyError::from_message("captcha_incorrect").into());
    }
  }

  let _alias = ext_name.clone();
  let _alias_id = ea.extra.clone();
  let mut _new_user = info.username.clone().to_lowercase().to_owned();
  let _new_password = info.password.to_owned();
  let mut _person_id: PersonId;
  let mut _exist = false;

  let other_person = match Person::find_by_extra_name(context.pool(), &_alias.clone()).await
  {
    Ok(c) => {
      _person_id = c.id.clone();
      _new_user = c.name.clone();
      _exist = true;
      let mut ret = Some(c);
      if kyced {
        match Person::update_kyced(context.pool(), _person_id).await
        {
          Ok(p) => { 
            ret = Some(p);            
          },
          Err(_e) => {            
          },
        };
      }
      ret      
    },
    Err(_e) => None,
  };

  if !_exist {
    let slur_regex = local_site_to_slur_regex(&local_site);
    check_slurs(&_new_user.clone(), &slur_regex)?;
    //check_slurs_opt(&info.answer, &slur_regex)?;
  }
  
  let actor_keypair = generate_actor_keypair()?;
  if !_exist {
    if !is_valid_actor_name(&_new_user.clone(), local_site.actor_name_max_length as usize) {
      println!(
        "Invalid username {} {}",
        ext_name.to_owned(),
        &_new_user.clone()
      );
      return Err(LemmyError::from_message("register:invalid_username"));
    }
  }

  let actor_id = generate_local_apub_endpoint(
    EndpointType::Person,
    &_new_user.clone(),
    &context.settings().get_protocol_and_hostname(),
  )?;

  let person = match Person::find_by_name(context.pool(), &_new_user.clone()).await
  {
    Ok(c) => Some(c),
    Err(_e) => None,
  };

  let mut change_password = false;
  match other_person {
    Some(ref op) => {
      //person_id = op.id.clone();
      //_exist = true;
      //change_password = true;
      match person {
        Some(other) => {
          if op.external_name != other.external_name {
            let err_type = format!(
              "External user {} is exist and belong to other account ",
              &_new_user.clone()
            );
            //println!("{} {} {}", name.clone(), &_new_user.clone(), err_type);
            return Err(LemmyError::from_message(&err_type).into());
          } else {
            // Same name and account: change password ???
            change_password = true;
          }
        }
        None => {
          change_password = true;
          let err_type = format!("External user: You already have user name {}, change password", op.name);
          //println!("{} {} {}", name.clone(), &_new_user.clone(), err_type);
        }
      };
    }
    None => {
      match person {
        Some(_other) => {
          let err_type = format!(
            "User {} is exist and belong to other user",
            &_new_user.clone()
          );
          //println!("{} {} {}", _alias.clone(), err_type, &_new_user.clone());
          return Err(LemmyError::from_message(&err_type).into());
        }
        None => {
          // No account relate with web3_address/site_username, we must completed the registration and create new user
        }
      };
    }
  }

  // Person is exist, change his password
  if change_password && other_person.is_some() {
    let _password_hash =
      hash(_new_password.clone(), DEFAULT_COST).expect("Couldn't hash password");

    //let _new_user = other_person.clone().unwrap().name.clone();
    //let _local_user = match LocalUserView::read_from_name(context.pool(), &_new_user.clone()).await
    let person_id = other_person.unwrap().id;
    let _local_user = match LocalUserView::read_person(context.pool(), person_id.clone()).await
    {
      Ok(lcu) => lcu,
      Err(_e) => {
        let err_type = format!(
          "External: Update local user not found {} {} {}",
          &_new_user.clone(),
          &ext_name.clone(),
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
          "External: Update local user password error {} {} {}",
          &_new_user.clone(),
          &ext_name.clone(),
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

  match Community::read_from_name(context.pool(), &_new_user.clone(), true).await
  {
    Ok(comm) => {
      return Err(LemmyError::from_message(
        "Cannot create user with community's name is exists",
      ));
    }
    Err(e) => {}
  };

  // We have to create both a person, and local_user
  // Register the new person
  let person_form = PersonInsertForm::builder()
    .name(_new_user.clone())
    .actor_id(Some(actor_id.clone()))
    .private_key(Some(actor_keypair.private_key))
    .public_key(actor_keypair.public_key)
    .inbox_url(Some(generate_inbox_url(&actor_id)?))
    .shared_inbox_url(Some(generate_shared_inbox_url(&actor_id)?))
    // If its the initial site setup, they are an admin
    .admin(Some(!local_site.site_setup))
    .instance_id(site_view.site.instance_id)
    .external_id(_alias_id.clone())
    .external_name(Some(_alias.clone().to_owned()))
    .verified(kyced)
    .build();


  let inserted_person = match Person::create(context.pool(), &person_form).await
  {
    Ok(p) => p,
    Err(_e) => {
      let err_type = format!(
        "External: user_already_exists: {} {}, exists{},  err:{}",
        &_new_user.clone(),
        _alias.clone(),
        _exist,
        _e.to_string()
      );
      return Err(LemmyError::from_message(&err_type).into());
    }
  };

  // Create the balance 
  let balance_form = PersonBalanceInsertForm::builder()
    .person_id(inserted_person.id.clone())
    .asset(Some("PI".to_string()))
    .deposited(0.0)
    .received(0.0)
    .spent(0.0)
    .amount(0.0)
    .withdrawed(0.0)
    .pending(0.0)
    .build();

  let inserted_balance = match PersonBalance::create(context.pool(), &balance_form).await {
    Ok(lu) => Some(lu),
    Err(e) => {
      None
    }
  };
    
  // Create the local user
  let local_user_form = LocalUserInsertForm::builder()
    .person_id(inserted_person.id)
    .email(info.email.as_deref().map(str::to_lowercase))
    .password_encrypted(info.password.to_string())
    .show_nsfw(Some(info.show_nsfw))
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

  if local_site.site_setup && require_application {
    // Create the registration application
    let form = RegistrationApplicationInsertForm {
      local_user_id: inserted_local_user.id,
      // We already made sure answer was not null above
      answer: info.answer.clone().expect("must have an answer"),
    };

    RegistrationApplication::create(context.pool(), &form).await?;
  }

  // Email the admins
  if local_site.application_email_admins {
    send_new_applicant_email_to_admins(&_new_user.clone(), context.pool(), context.settings())
      .await?;
  }

  let mut login_response = LoginResponse {
    jwt: None,
    registration_created: false,
    verify_email_sent: false,
  };

  // Log the user in directly if the site is not setup, or email verification and application aren't required
  if !local_site.site_setup
    || (!require_application && !local_site.require_email_verification)
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

    if require_application {
      login_response.registration_created = true;
    }
  }
  return Ok(login_response);
}
