use bcrypt::{hash, DEFAULT_COST};
use crate::pipayment::client::*;
use crate::PerformCrud;
use actix_web::web::Data;
use lemmy_api_common::{utils::{blocking, password_length_check,}, person::*, pipayment::*, sensitive::Sensitive};
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
  traits::{Crud, ApubActor, Followable, },
  newtypes::{CommunityId, PaymentId, PersonId,}, 
  schema::local_user::email_verified,
};
use lemmy_db_views::{structs::LocalUserView};
use lemmy_db_views_actor::structs::PersonViewSafe;

use lemmy_utils::{
  apub::generate_actor_keypair,
  claims::Claims,
  settings::SETTINGS,
  utils::{check_slurs, is_valid_actor_name,},
  error::LemmyError,
  ConnectionId,
};
use lemmy_websocket::{LemmyContext};
use sha2::{Digest, Sha256, };
use uuid::Uuid;

#[async_trait::async_trait(?Send)]
impl PerformCrud for PiLogin {
  type Response = LoginResponse;

  async fn perform(
    &self,
    context: &Data<LemmyContext>,
    _websocket_id: Option<ConnectionId>,
  ) -> Result<LoginResponse, LemmyError> {
    // Call from client after Pi.authenticate

    let data: &PiLogin = &self;
    let settings = SETTINGS.to_owned();
    // Make sure site has open registration
    if let Ok(site) = blocking(context.pool(), move |conn| Site::read_local_site(conn)).await? {
      if !site.open_registration {
        return Err(LemmyError::from_message("registration_closed"));
      }
    }
    if !settings.pinetwork.pi_free_login {
      //return Err(LemmyError::from_message("registration_closed"));
    }
    // Hide Pi user name, not store pi_uid
    let mut _pi_username = data.pi_username.clone();
    let mut _pi_uid = data.pi_uid.clone();
    let _pi_token = data.pi_token.clone();

    println!("PiLogin is processing for {} {} {} ", _pi_uid.clone(), _pi_username.clone(), _pi_token.clone());

    // First, valid user token
    let user_dto = match pi_me(context.client(), &data.pi_token.clone()).await {
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
        create_new =  true;
        _new_user = info.username_or_email.clone();
        _new_password =  info.password.clone();
      },
      None =>{
        
      }
    }

    // Check if there are admins. False if admins exist
    let no_admins = blocking(context.pool(), move |conn| {
         PersonViewSafe::admins(conn).map(|a| a.is_empty())
    })
    .await??;


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
    let pi_person = match blocking(context.pool(), move |conn| {
      Person::find_by_pi_name(&conn, &_pi_alias)
    })
    .await?
    {
      Ok(c) => Some(c),
      Err(_e) => None,
    };

  
    let mut extra_user_id = None;
    match pi_person {
      Some(pi) => {
        pi_exist = true;
        person_id = pi.id;
        username = pi.name.clone();
        extra_user_id = pi.extra_user_id;
      }
      None => {
        if !create_new {
          let err_type = format!("Hi {}, you must register before login.", &username);
          println!("{} {}", _pi_uid.clone(), err_type);
          return Err(LemmyError::from_message(&err_type));
        }
      }
    }
    
    if pi_exist {      
       let local_user_id;
       let username2 = username.clone();
       let _local_user = match blocking(context.pool(), move |conn| {
         LocalUserView::read_from_name(&conn, &username2.clone())
       })
       .await?
       {
         Ok(lcu) => lcu, 
         Err(_e) => {
           let err_type = format!("PiLogin local user not found {} {} {}", _pi_username.clone(), username.clone(),  _e.to_string());
           println!("{} {}", _pi_uid.clone(), err_type);
           return Err(LemmyError::from_error_message(_e, &err_type));
        
          //  return Ok(PiRegisterResponse {
          //   success: false,
          //   jwt: format!(""),
          //   extra: Some(format!("{}",err_type)),
          //   });
         }
       };

       local_user_id = _local_user.local_user.id.clone();

      //  let password_hash = hash(_new_password.clone(), DEFAULT_COST).expect("Couldn't hash password");
      if create_new {
        let updated_local_user = match blocking(context.pool(), move |conn| {
          LocalUser::update_password(&conn, local_user_id.clone(), &_new_password)
        })
        .await
        {
          Ok(lcu) => lcu,
          Err(_e) => {
            let err_type = format!("PiLogin: Update local user password error {} {}", &username.clone(), _e.to_string());
            return Err(LemmyError::from_message(&err_type));
            }
        };
      }
      // let _pi_uid_search = _pi_uid.clone();
      // let  _payment = match blocking(context.pool(), move |conn| {
      //   PiPayment::find_by_pi_uid(&conn, &_pi_uid_search)
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
      })
          
    } // Pi exist


    // We have to create both a person, and local_user
    if !create_new {
      let err_type = format!("Auto create new account for Pioneers is disabled {} {}", &_new_user.to_string().clone(), &_pi_uid.clone());
      println!("{}", err_type);
      //return LemmyError::from_error_message(e, &err_type)?;
      return Err(LemmyError::from_message(&err_type).into());
    }

    check_slurs(&_new_user.clone(), &context.settings().slur_regex())?;
    if !is_valid_actor_name(&_new_user.clone(), context.settings().actor_name_max_length) {
        //println!("Invalid username {} {}", _pi_username.to_owned(), &_new_user.clone());
        //return LemmyError::from_error_message(e, &err_type)?;
        return Err(LemmyError::from_message("register:invalid_username").into());
    }  

    let mut change_password = false;

    let _new_user2 = _new_user.clone();
    let person = match blocking(context.pool(), move |conn| {
      Person::find_by_name(&conn, &_new_user2.clone())
    })
    .await?
    {
      Ok(c) => Some(c),
      Err(_e) => None,
    };

    match person {
      Some(per) => {
        if extra_user_id != per.extra_user_id {
          let err_type = format!("User {} is exist and belong to other Pi Account ", &_new_user.to_string().clone());
          println!("{} {} {}", _pi_username.clone(), err_type, &_pi_alias2);
          result = false;
          //return LemmyError::from_error_message(e, &err_type)?;
          //return Err(LemmyError::from_message(&err_type).into());
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
        //change_username = true;
        // Not allow change username
        //let err_type = format!("Register: You already have user name {}", _new_user.clone());
        //println!("{} {} {}", data.pi_username.clone(), err_type, &_pi_alias2);
        //result = false;
        //return Err(LemmyError::from_message(&err_type).into());
        // return Ok(PiRegisterResponse {
        //   success: false,
        //   jwt: format!(""),
        //   extra: Some(format!("{}",err_type)),
        //   });
      }
    };

    let actor_keypair = generate_actor_keypair()?;
    let actor_id = generate_local_apub_endpoint(EndpointType::Person, &_new_user.clone(), &settings.get_protocol_and_hostname())?;

    // Register the new person
    let person_form = PersonForm {
      name: _new_user.to_string(),
      actor_id: Some(actor_id.clone()),
      private_key: Some(Some(actor_keypair.private_key)),
      public_key: actor_keypair.public_key,
      inbox_url: Some(generate_inbox_url(&actor_id)?),
      shared_inbox_url: Some(Some(generate_shared_inbox_url(&actor_id)?)),
      admin: None,
      extra_user_id: Some(_pi_alias2),
      ..PersonForm::default()
    };

    // insert the person
    // let err_type = format!("user_already_exists: {} {}", &data.info.username, _pi_alias3);
    let inserted_tmp = match blocking(context.pool(), move |conn| {
      Person::create(conn, &person_form)
    })
    .await?
    {
      Ok(p) => Some(p),
      Err(_e) => {
      let err_type = format!("PiLogin: user_already_exists: {} {}, exists{},  err:{}", 
                             &_new_user.to_string().clone(), _pi_alias3, pi_exist, _e.to_string());
      return Err(LemmyError::from_message(&err_type));
      },
    };


    let inserted_person = inserted_tmp.unwrap();
    // Create the local user
    let local_user_form = LocalUserForm {
      person_id: Some(inserted_person.id),
      email: None, //Some(info.email.to_owned()),
      password_encrypted: Some(_new_password.to_string()),
      show_nsfw: Some(false),
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
          Person::delete(&conn, inserted_person.id)
        })
        .await??;

        return Err(LemmyError::from_message(err_type));
      }
    };

    let main_community_keypair = generate_actor_keypair()?;

    // Create the main community if it doesn't exist
    let main_community = match blocking(context.pool(), move |conn| {
      Community::read_from_name(conn, "main", false)
    })
    .await?
    {
      Ok(c) => c,
      Err(_e) => {
        let default_community_name = "main";
        let actor_id = generate_local_apub_endpoint(EndpointType::Community, default_community_name, &settings.get_protocol_and_hostname())?;
        let community_form = CommunityForm {
          name: default_community_name.to_string(),
          title: "The Default Community".to_string(),
          description: Some("The Default Community".to_string()),
          actor_id: Some(actor_id.to_owned()),
          private_key: Some(Some(main_community_keypair.private_key)),
          public_key: main_community_keypair.public_key,
          followers_url: Some(generate_followers_url(&actor_id)?),
          inbox_url: Some(generate_inbox_url(&actor_id)?),
          shared_inbox_url: Some(Some(generate_shared_inbox_url(&actor_id)?)),
          ..CommunityForm::default()
        };
        blocking(context.pool(), move |conn| {
          Community::create(conn, &community_form)
        })
        .await??
      }
    };

    // Sign them up for main community no matter what
    let community_follower_form = CommunityFollowerForm {
      community_id: main_community.id,
      person_id: inserted_person.id,
      pending: false,
    };

    let follow = move |conn: &'_ _| CommunityFollower::follow(conn, &community_follower_form);
    if blocking(context.pool(), follow).await?.is_err() {
      //return Err(LemmyError::from_message("Register: community_follower_already_exists").into());
    };

    // If its an admin, add them as a mod and follower to main
    // if no_admins {
    //   let community_moderator_form = CommunityModeratorForm {
    //     community_id: main_community.id,
    //     person_id: inserted_person.id,
    //   };

    //   let join = move |conn: &'_ _| CommunityModerator::join(conn, &community_moderator_form);
    //   if blocking(context.pool(), join).await?.is_err() {
    //     return Err(LemmyError::from_message("community_moderator_already_exists").into());
    //   }
    // }

    // Return the jwt
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
