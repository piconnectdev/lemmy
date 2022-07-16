use bcrypt::{hash, DEFAULT_COST};
use crate::pipayment::client::*;
use crate::PerformCrud;
use actix_web::web::Data;
use lemmy_api_common::{utils::{blocking, honeypot_check, password_length_check,}, pipayment::*};
use lemmy_apub::{
  generate_local_apub_endpoint, generate_followers_url, generate_inbox_url, generate_shared_inbox_url,
  EndpointType,
};
use lemmy_db_schema::{
  utils::naive_now,
  source::{
    community::*,
    local_user::{LocalUser, LocalUserForm},
    person::*,
    pipayment::*,
    site::*,
  },
  impls::pipayment::PiPayment_,
  traits::{Crud, ApubActor, Followable, Joinable,  },
  newtypes::{CommunityId, PaymentId, PersonId,},
};
use lemmy_db_views::{structs::LocalUserView};
use lemmy_db_views_actor::{person_view::{*}, structs::PersonViewSafe};

use lemmy_utils::{
  apub::generate_actor_keypair,
  claims::Claims,
  settings::SETTINGS,
  utils::{check_slurs, is_valid_actor_name},
  error::{LemmyError},
  ConnectionId,
};
use lemmy_websocket::{messages::CheckCaptcha, LemmyContext};
use sha2::{Digest, Sha256};
use uuid::Uuid;

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

    let mut result = true;
    // Make sure site has open registration
    if let Ok(site) = blocking(context.pool(), move |conn| Site::read_local_site(conn)).await? {
      if !site.open_registration {
        return Err(LemmyError::from_message("registration_closed"));
      }
      email_verification = site.require_email_verification;
      require_application = site.require_application;
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
          uuid: data
                .token.clone(),
          answer: "".to_string(),
        })
        .await?;
      if !check {
        return Err(LemmyError::from_message("token_incorrect"));
      }
    }

    check_slurs(&data.info.username, &context.settings().slur_regex())?;

    let actor_keypair = generate_actor_keypair()?;
    if !is_valid_actor_name(&data.info.username, context.settings().actor_name_max_length) {
      println!("Invalid username {} {}", data.pi_username.to_owned(), &data.info.username);
      //return Err(LemmyError::from_message("register:invalid_username"));
      let err_type = format!("Register: invalid username {}", &data.info.username);
      return Ok(PiRegisterResponse {
        success: false,
        jwt: format!(""),
        extra: Some(format!("{}",err_type)),
        });
    }
    let actor_id = generate_local_apub_endpoint(
      EndpointType::Person,
      &data.info.username, 
      &settings.get_protocol_and_hostname())?;

    // Hide Pi user name, not store pi_uid
    let mut sha256 = Sha256::new();
    sha256.update(settings.pi_seed());
    sha256.update(data.pi_username.to_owned());
    let _pi_alias: String = format!("{:X}", sha256.finalize());
    let _pi_alias2 = _pi_alias.clone();
    let _pi_alias3 = _pi_alias.clone();
    //let _pi_alias = data.pi_username.to_owned();

    let _pi_uid = data.pi_uid.clone();
    let _payment_id = data.paymentid.to_owned();
    let _new_user = data.info.username.to_owned();
    let _new_user2 = data.info.username.to_owned();
    let _new_password = data.info.password.to_owned();

    let mut approved = false;
    let mut completed = false;
    let mut finished = false;
    let payment_id: PaymentId;
    let person_id: PersonId;
    let mut pi_exist = false;

    let person = match blocking(context.pool(), move |conn| {
      Person::find_by_name(&conn, &_new_user)
    })
    .await?
    {
      Ok(c) => Some(c),
      Err(_e) => None,
    };

    let other_person = match blocking(context.pool(), move |conn| {
      Person::find_by_web3_address(&conn, &_pi_alias)
    })
    .await?
    {
      Ok(c) => Some(c),
      Err(_e) => None,
    };

    let mut change_password = false;
    let mut change_username = false;
    match other_person {
      Some(op) => {
        person_id = op.id;
        _exist = true;
        match person {
          Some(other) => {
            if op.extra_user_id != other.extra_user_id {
              let err_type = format!("Web3Register: User {} is exist and belong to other Web3 Account ", &data.info.username);
              println!("{} {} {}", data.address.clone(), err_type, &_alias2);
              result = false;
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
            change_username = true;
            // Not allow change username
            let err_type = format!("PiRegister: You already have user name {}", op.name);
            println!("{} {} {}", data.address.clone(), err_type, &_alias2);
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
            let err_type = format!("Web3Register: User {} is exist and belong to other user", &data.info.username);
            println!("{} {} {}", data.pi_username.clone(), err_type, &_pi_alias2);
            result = false;
            return Err(LemmyError::from_message(&err_type).into());
            // return Ok(PiRegisterResponse {
            //   success: false,
            //   jwt: format!(""),
            //   extra: Some(format!("{}",err_type)),
            //   });
          }
          None => {
            // No account relate with pi_username/site_username, we must completed the payment and create new user
          }
        };
      }
    }

    /// TODO: UUID check
    let refid = match &data.info.captcha_uuid {
      Some(uid) => match Uuid::parse_str(uid) {
        Ok(uidx) => Some(uidx),
        Err(_e) => None,
      },
      None => None,
    };

    
    // Person is exist, change his password
    if change_password {

      let password_hash = hash(_new_password.clone(), DEFAULT_COST).expect("Couldn't hash password");
      
       let local_user_id;
       let _local_user = match blocking(context.pool(), move |conn| {
         LocalUserView::read_from_name(&conn, &_new_user2)
       })
       .await?
       {
         Ok(lcu) => lcu, 
         Err(_e) => {
           let err_type = format!("Register: Update local user not found {} {} {}", &data.info.username, &data.paymentid, _e.to_string());
           return Err(LemmyError::from_message(&err_type).into());
          //  return Ok(PiRegisterResponse {
          //   success: false,
          //   jwt: format!(""),
          //   extra: Some(format!("{}",err_type)),
          //   });
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
           let err_type = format!("Register: Update local user password error {} {} {}", &data.info.username, &data.paymentid, _e.to_string());
           return Err(LemmyError::from_message(&err_type).into());
          //  Ok(LoginResponse {
          //   jwt: Some(
          //     Claims::jwt(
          //       inserted_local_user.id.0,
          //       &context.secret().jwt_secret,
          //       &context.settings().hostname,
          //     )?
          //     .into(),
          //   ),
          //   verify_email_sent: false,
          //   registration_created: false,
          // })
       };
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
    // let err_type = format!("user_already_exists: {} {}", &data.info.username, _pi_alias3);
    let inserted_person = match blocking(context.pool(), move |conn| {
      Person::create(conn, &person_form)
    })
    .await?
    {
      Ok(p) => p,
      Err(_e) => {
      let err_type = format!("Register: user_already_exists: {} {}, exists{},  err:{}", 
                             &data.info.username, _pi_alias3, pi_exist, _e.to_string());
      return Err(LemmyError::from_message(&err_type).into());
      },
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
          "Register: email_already_exists"
        } else {
          "Register: user_already_exists"
        };

        // If the local user creation errored, then delete that person
        blocking(context.pool(), move |conn| {
          Person::delete(&conn, inserted_person.id.clone())
        })
        .await??;

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
  }
}
