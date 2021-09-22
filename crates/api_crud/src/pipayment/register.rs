use bcrypt::{hash, DEFAULT_COST};
use crate::pipayment::client::*;
use crate::PerformCrud;
use actix_web::web::Data;
use lemmy_api_common::{blocking, password_length_check, person::*, pipayment::*};
use lemmy_apub::{
  generate_apub_endpoint, generate_followers_url, generate_inbox_url, generate_shared_inbox_url,
  EndpointType,
};
use lemmy_db_queries::{
  source::{community::Community_, local_user::LocalUser_, person::*, pipayment::*, site::Site_},
  Crud, Followable, Joinable, ListingType, SortType,
};
use lemmy_db_schema::{
  naive_now,
  source::{
    community::*,
    local_user::{LocalUser, LocalUserForm},
    person::*,
    pipayment::*,
    site::*,
  },
  CommunityId, PaymentId, PersonId,
};
use lemmy_db_views::{comment_view::CommentView, local_user_view::LocalUserView};
use lemmy_db_views_actor::person_view::PersonViewSafe;

use lemmy_utils::{
  apub::generate_actor_keypair,
  claims::Claims,
  settings::structs::Settings,
  utils::{check_slurs, is_valid_actor_name},
  ApiError, ConnectionId, LemmyError,
};
use lemmy_websocket::{messages::CheckCaptcha, LemmyContext};
use sha2::{Digest, Sha256, Sha512};
use uuid::Uuid;

#[async_trait::async_trait(?Send)]
impl PerformCrud for PiRegister {
  type Response = PiRegisterResponse;

  async fn perform(
    &self,
    context: &Data<LemmyContext>,
    _websocket_id: Option<ConnectionId>,
  ) -> Result<PiRegisterResponse, LemmyError> {
    let data: &PiRegister = &self;

    // Make sure site has open registration
    if let Ok(site) = blocking(context.pool(), move |conn| Site::read_simple(conn)).await? {
      if !site.open_registration {
        return Err(ApiError::err("registration_closed").into());
      }
    }

    password_length_check(&data.info.password)?;

    // Make sure passwords match
    if data.info.password != data.info.password_verify {
      return Err(ApiError::err("passwords_dont_match").into());
    }

    // Check if there are admins. False if admins exist
    let no_admins = blocking(context.pool(), move |conn| {
      PersonViewSafe::admins(conn).map(|a| a.is_empty())
    })
    .await??;

    // If its not the admin, check the captcha
    if !no_admins && Settings::get().captcha.enabled {
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
        return Err(ApiError::err("captcha_incorrect").into());
      }
    }

    check_slurs(&data.info.username)?;

    let actor_keypair = generate_actor_keypair()?;
    if !is_valid_actor_name(&data.info.username) {
      return Err(ApiError::err("register:invalid_username").into());
    }
    let actor_id = generate_apub_endpoint(EndpointType::Person, &data.info.username)?;

    // Hide Pi user name, not store pi_uid
    let mut sha256 = Sha256::new();
    sha256.update(Settings::get().pi_seed());
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
    let mut payment_id: PaymentId;
    let mut person_id: PersonId;
    let mut pi_exist = false;
    let mut dto: Option<PiPaymentDto> = None;

    let mut _payment = match blocking(context.pool(), move |conn| {
      PiPayment::find_by_pipayment_id(&conn, &_payment_id)
    })
    .await?
    {
      Ok(c) => {
        approved = c.approved;
        completed = c.completed;
        payment_id = c.id;
        finished = c.finished;
        Some(c)
      }
      Err(_e) => {
        //let err_type = format!("Payment {} was not approved", data.paymentid);
        let err_type = format!("Register: Payment {} was not approved, err: {}", data.paymentid, _e.to_string());
        return Err(ApiError::err(&err_type).into());
      }
    };

    if _payment.is_none() {
      // Why here ????
      let err_type = format!("Register: Payment {} was not insert/approved", data.paymentid);
      return Err(ApiError::err(&err_type).into());
    } else {
      if finished {
        let err_type = format!("Register: Payment {} was finished", data.paymentid);
        return Err(ApiError::err(&err_type).into());
      }
    }

    if !completed {
      dto = match pi_complete(
        context.client(),
        &data.paymentid.clone(),
        &data.txid.clone(),
      )
      .await
      {
        Ok(c) => Some(c),
        Err(_e) => {
          // Server error
          let err_type = format!("Register: Pi Server API complete error: {} {} {}", &data.info.username, &data.paymentid, _e.to_string());
          //let err_type = _e.to_string();
          //return Err(ApiError::err(&err_type).into());
          return Ok(PiRegisterResponse {
            success: false,
            jwt: format!(""),
            extra: Some(format!("Register: Pi Server API complete error: {} {} {}", &data.info.username, &data.paymentid, _e.to_string())),
            });
        }
      };
    }

    let pi_person = match blocking(context.pool(), move |conn| {
      Person::find_by_pi_name(&conn, &_pi_alias)
    })
    .await?
    {
      Ok(c) => Some(c),
      Err(_e) => None,
    };

    let person = match blocking(context.pool(), move |conn| {
      Person::find_by_name(&conn, &_new_user)
    })
    .await?
    {
      Ok(c) => Some(c),
      Err(_e) => None,
    };

    let mut change_password = false;
    let mut change_username = false;
    match pi_person {
      Some(pi) => {
        person_id = pi.id;
        pi_exist = true;
        match person {
          Some(per) => {
            if pi.extra_user_id != per.extra_user_id {
              let err_type = format!(
                "Register: WePi user {} is exist and belong to other Pi account {}",
                &data.info.username,  &_pi_alias2
              );
              //return Err(ApiError::err(&err_type).into());
              return Ok(PiRegisterResponse {
                success: false,
                jwt: format!(""),
                extra: Some(format!("{}",err_type)),
                });
            } else {
              // Same name and account: change password ???
              change_password = true;
            }
          }
          None => {
            change_password = true;
            change_username = true;
            // Not allow change username
            let err_type = format!("Register: Account already have user name {} {}", pi.name, &_pi_alias2);
            //return Err(ApiError::err(&err_type).into());
            return Ok(PiRegisterResponse {
              success: false,
              jwt: format!(""),
              extra: Some(format!("{}",err_type)),
              });
          }
        };
      }
      None => {
        match person {
          Some(per) => {
            let err_type = format!("Register: WePi user {} is exist {}", &data.info.username,  &_pi_alias2);
            //return Err(ApiError::err(&err_type).into());
            return Ok(PiRegisterResponse {
              success: false,
              jwt: format!(""),
              extra: Some(format!("{}",err_type)),
              });
          }
          None => {
            // No account, we must completed this and create new user
          }
        };
      }
    }
    
    let mut _payment_dto = PiPaymentDto {
      ..PiPaymentDto::default()
    };
    _payment_dto.status.developer_approved  =  true;
    _payment_dto.status.developer_completed =  true;
    _payment_dto.status.transaction_verified=  true;
    if dto.is_some() {
      _payment_dto = dto.unwrap();
    }

    let refid = match &data.info.captcha_uuid {
      Some(uid) => match Uuid::parse_str(uid) {
        Ok(uidx) => Some(uidx),
        Err(_e) => None,
      },
      None => None,
    };

    let create_at = match chrono::NaiveDateTime::parse_from_str(&_payment_dto.created_at, "%Y-%m-%dT%H:%M:%S%.f%Z"){
      Ok(dt) => Some(dt),
      Err(_e) => {
        let err_type = format!("Register: Pi Server: get payment datetime error: user {}, paymentid {} {} {}", 
        &data.pi_username, &data.paymentid, _payment_dto.created_at, _e.to_string() );
        //return Err(ApiError::err(&err_type).into());
        None  
      }
    };

    // Update relate payment
    let mut payment_form = PiPaymentForm {
      person_id: None,
      ref_id: refid,
      testnet: Settings::get().pi_testnet,
      finished: true,
      updated: Some(naive_now()),
      pi_uid: data.pi_uid,
      pi_username: "".to_string(), // data.pi_username.clone(), Hide username
      comment: data.comment.clone(),

      identifier: _payment_dto.identifier, //data.paymentid
      user_uid: _payment_dto.user_uid,
      amount: _payment_dto.amount,
      memo: _payment_dto.memo,
      to_address: _payment_dto.to_address,
      created_at: create_at,
      approved: _payment_dto.status.developer_approved,
      verified: _payment_dto.status.transaction_verified,
      completed: _payment_dto.status.developer_completed,
      cancelled: _payment_dto.status.cancelled,
      user_cancelled: _payment_dto.status.user_cancelled,
      tx_link: "".to_string(),
      tx_id: "".to_string(),
      tx_verified: false,
      metadata: _payment_dto.metadata,
      extras: None,
      //tx_id:  _payment_dto.transaction.map(|tx| tx.txid),
      //..PiPaymentForm::default()
    };

    match _payment_dto.transaction {
      Some(tx) => {
        payment_form.tx_link = tx._link;
        payment_form.tx_verified = tx.verified;
        payment_form.tx_id = tx.txid;
      }
      None => {}
    }

    let updated_payment = match blocking(context.pool(), move |conn| {
      PiPayment::update(&conn, payment_id, &payment_form)
    })
    .await?
    {
      Ok(payment) => payment,
      Err(_e) => {
        let err_type = format!("Register: Update payment complete error: {} {} {}", &data.info.username, &data.paymentid, _e.to_string());
        //return Err(ApiError::err(&err_type).into());
        return Ok(PiRegisterResponse {
          success: false,
          jwt: format!(""),
          extra: Some(format!("{}",err_type)),
          });
      }
    };
    
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
           //return Err(ApiError::err(&err_type).into());
           return Ok(PiRegisterResponse {
            success: false,
            jwt: format!(""),
            extra: Some(format!("{}",err_type)),
            });
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
           //return Err(ApiError::err(&err_type).into());
           return Ok(PiRegisterResponse {
            success: false,
            jwt: format!(""),
            extra: Some(format!("{}",err_type)),
            });
            }
       };
        return Ok(PiRegisterResponse {
            success: true,
            jwt: Claims::jwt(local_user_id.0)?,
            extra: None,
            })
    }
    // We have to create both a person, and local_user

    // Register the new person
    let person_form = PersonForm {
      name: data.info.username.to_owned(),
      actor_id: Some(actor_id.clone()),
      private_key: Some(Some(actor_keypair.private_key)),
      public_key: Some(Some(actor_keypair.public_key)),
      inbox_url: Some(generate_inbox_url(&actor_id)?),
      shared_inbox_url: Some(Some(generate_shared_inbox_url(&actor_id)?)),
      admin: Some(no_admins),
      extra_user_id: Some(_pi_alias2),
      ..PersonForm::default()
    };

    // insert the person
    // let err_type = format!("user_already_exists: {} {}", &data.info.username, _pi_alias3);
    let inserted_person1 = match blocking(context.pool(), move |conn| {
      Person::create(conn, &person_form)
    })
    .await?
    {
      Ok(p) => Some(p),
      Err(_e) => {
      let err_type = format!("Register: user_already_exists: {} {}, exists{},  err:{}", 
                             &data.info.username, _pi_alias3, pi_exist, _e.to_string());
      return Err(ApiError::err(&err_type).into());
      },
    };


    let inserted_person = inserted_person1.unwrap();
    // Create the local user
    let local_user_form = LocalUserForm {
      person_id: inserted_person.id,
      email: Some(data.info.email.to_owned()),
      password_encrypted: data.info.password.to_owned(),
      show_nsfw: Some(data.info.show_nsfw),
      show_bot_accounts: Some(true),
      theme: Some("browser".into()),
      default_sort_type: Some(SortType::Active as i16),
      default_listing_type: Some(ListingType::Subscribed as i16),
      lang: Some("browser".into()),
      show_avatars: Some(true),
      show_scores: Some(true),
      show_read_posts: Some(true),
      send_notifications_to_email: Some(false),
      show_new_post_notifs: Some(false),
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

        return Err(ApiError::err(err_type).into());
      }
    };

    let main_community_keypair = generate_actor_keypair()?;

    // Create the main community if it doesn't exist
    let main_community = match blocking(context.pool(), move |conn| {
      Community::read_from_name(conn, "main")
    })
    .await?
    {
      Ok(c) => c,
      Err(_e) => {
        let default_community_name = "main";
        let actor_id = generate_apub_endpoint(EndpointType::Community, default_community_name)?;
        let community_form = CommunityForm {
          name: default_community_name.to_string(),
          title: "The Default Community".to_string(),
          description: Some("The Default Community".to_string()),
          actor_id: Some(actor_id.to_owned()),
          private_key: Some(main_community_keypair.private_key),
          public_key: Some(main_community_keypair.public_key),
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
      //return Err(ApiError::err("Register: community_follower_already_exists").into());
    };

    // If its an admin, add them as a mod and follower to main
    if no_admins {
      let community_moderator_form = CommunityModeratorForm {
        community_id: main_community.id,
        person_id: inserted_person.id,
      };

      let join = move |conn: &'_ _| CommunityModerator::join(conn, &community_moderator_form);
      if blocking(context.pool(), join).await?.is_err() {
        return Err(ApiError::err("community_moderator_already_exists").into());
      }
    }

    // Return the jwt
    Ok(PiRegisterResponse {
      success: true,
      jwt: Claims::jwt(inserted_local_user.id.0)?,
      extra: None,
    })
  }
}
