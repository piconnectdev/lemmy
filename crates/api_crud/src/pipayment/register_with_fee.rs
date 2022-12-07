use crate::PerformCrud;
use actix_web::web::Data;
use lemmy_api_common::{context::LemmyContext};
use lemmy_api_common::{
  pipayment::*,
};
use lemmy_db_schema::{
  source::{*
  },
};
use lemmy_db_views::structs::SiteView;
use lemmy_utils::{
  error::LemmyError,
  settings::SETTINGS,
  ConnectionId,
};
use crate::web3::ext::*;

use super::client::pi_payment_update;

#[async_trait::async_trait(?Send)]
impl PerformCrud for PiRegisterWithFee {
  type Response = PiRegisterResponse;
  async fn perform(
    &self,
    context: &Data<LemmyContext>,
    _websocket_id: Option<ConnectionId>,
  ) -> Result<PiRegisterResponse, LemmyError> {
    let settings = SETTINGS.to_owned();
    let data: &PiRegisterWithFee = &self;
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
        println!("PiRegisterWithFee: not pi_enabled: {} ", data.paymentid.clone());
        return Err(LemmyError::from_message("registration_disabled"));
      }
      // if !settings.pinetwork.pi_allow_all {
      //   return Err(LemmyError::from_message("registration_disabled"));
      // }
    }
    let payment_id = data.ea.signature.clone().unwrap_or_default();
    // TODO: Check paymentid complete
    let approve = PiApprove {
      paymentid: payment_id.clone(),
      pi_username: ext_account.account.clone(),
      pi_uid: None, //ext_account.puid.clone(),
      person_id: None,
      comment: None,
      auth: None,
    };

    let payment = match pi_payment_update(context, &approve.clone(), Some(data.txid.clone())).await
    {
      Ok(p) => {
        if !p.completed {
          println!("PiRegisterWithFee: not completed: {} ", p.identifier.clone());
          return Err(LemmyError::from_message("registration_disabled"));
        }
        Some(p)
      },
      Err(_c) => {
        println!("PiRegisterWithFee: pi_payment_update: {} ", _c.to_string());
        return Err(LemmyError::from_message("registration_disabled"));
      },
    };

    let login_response = match create_external_account(context, &ext_account.account.clone(), &ext_account.clone(), &data.info.clone()).await
    {
      Ok(c) => c,
      Err(_e) => {
        println!("PiRegisterWithFee: create_external_account: {} {} {}", data.paymentid.clone(), &ext_account.account.clone(), &data.info.username.clone());
        return Err(LemmyError::from_message("registration_disabled"));
        //None
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
    // if !no_admins && settings.captcha.enabled {
    //   let check = context
    //     .chat_server()
    //     .send(CheckCaptcha {
    //       uuid: data
    //         .info
    //         .captcha_uuid
    //         .to_owned()
    //         .unwrap_or_else(|| "".to_string()),
    //       answer: data
    //         .info
    //         .captcha_answer
    //         .to_owned()
    //         .unwrap_or_else(|| "".to_string()),
    //     })
    //     .await?;
    //   if !check {
    //     return Err(LemmyError::from_message("captcha_incorrect").into());
    //   }
    // }

    check_slurs(&data.info.username, &context.settings().slur_regex())?;

    let actor_keypair = generate_actor_keypair()?;
    if !is_valid_actor_name(&data.info.username, local_site.actor_name_max_length as usize)  {
      println!(
        "Invalid username {} {}",
        ext_account.account.to_owned(),
        &data.info.username
      );
      //return Err(LemmyError::from_message("register:invalid_username"));
      let err_type = format!("Register: invalid username {}", &data.info.username);
      return Ok(PiRegisterResponse {
        success: false,
        jwt: format!(""),
        extra: Some(format!("{}", err_type)),
      });
    }
    let actor_id = generate_local_apub_endpoint(
      EndpointType::Person,
      &data.info.username,
      &settings.get_protocol_and_hostname(),
    )?;

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
    let payment_id: PiPaymentId;
    let person_id: PersonId;
    let mut pi_exist = false;
    let mut dto: Option<PiPaymentDto> = None;

    let mut _payment = match blocking(context.pool(), move |conn| {
      PiPayment::find_by_pipayment_id(conn, &_payment_id)
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
        let err_type = format!(
          "PiRegister: Payment {} was not approved, err: {}",
          data.paymentid,
          _e.to_string()
        );
        //return Err(LemmyError::from_message(&err_type));
        return Ok(PiRegisterResponse {
          success: false,
          jwt: format!(""),
          extra: Some(format!("{}", err_type)),
        });
      }
    };

    if _payment.is_none() {
      // Why here ????
      let err_type = format!(
        "PiRegister: Payment {} was not insert/approved",
        data.paymentid
      );
      return Err(LemmyError::from_message(&err_type).into());
    } else {
      if finished {
        let err_type = format!("PiRegister: Payment {} was finished", data.paymentid);
        return Err(LemmyError::from_message(&err_type).into());
      }
    }

    let pi_person = match blocking(context.pool(), move |conn| {
      Person::find_by_pi_name(conn, &_pi_alias)
    })
    .await?
    {
      Ok(c) => Some(c),
      Err(_e) => None,
    };

    let person = match blocking(context.pool(), move |conn| {
      Person::find_by_name(conn, &_new_user)
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
          Some(other) => {
            if pi.external_id != other.external_id {
              let err_type = format!(
                "Register: User {} is exist and belong to other Pi Account ",
                &data.info.username
              );
              println!("{} {} {}", data.pi_username.clone(), err_type, &_pi_alias2);
              result = false;
              //return Err(LemmyError::from_message(&err_type).into());
              return Ok(PiRegisterResponse {
                success: false,
                jwt: format!(""),
                extra: Some(format!("{}", err_type)),
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
            //return Err(LemmyError::from_message(&err_type).into());
            return Ok(PiRegisterResponse {
              success: false,
              jwt: format!(""),
              extra: Some(format!("{}", err_type)),
            });
          }
          None => {
            // No account relate with pi_username/site_username, we must completed the payment and create new user
          }
        };
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
          let err_type = format!(
            "PiRegister: Pi Server API complete the payment error: {} {} {}",
            &data.info.username,
            &data.paymentid,
            _e.to_string()
          );
          //return Err(LemmyError::from_message(&err_type).into());
          return Ok(PiRegisterResponse {
            success: false,
            jwt: format!(""),
            extra: Some(format!("{}", err_type)),
          });
        }
      };
    }

    let mut _payment_dto = PiPaymentDto {
      ..PiPaymentDto::default()
    };
    _payment_dto.status.developer_approved = true;
    _payment_dto.status.developer_completed = true;
    _payment_dto.status.transaction_verified = true;
    if dto.is_some() {
      _payment_dto = dto.unwrap();
    }

    /// TODO: UUID check
    let refid = match &data.info.captcha_uuid {
      Some(uid) => match Uuid::parse_str(uid) {
        Ok(uidx) => Some(uidx),
        Err(_e) => None,
      },
      None => None,
    };

    let create_at = match chrono::NaiveDateTime::parse_from_str(
      &_payment_dto.created_at,
      "%Y-%m-%dT%H:%M:%S%.f%Z",
    ) {
      Ok(dt) => Some(dt),
      Err(_e) => {
        let err_type = format!(
          "PiRegister: Pi Server: get payment datetime error: user {}, paymentid {} {} {}",
          &data.pi_username,
          &data.paymentid,
          _payment_dto.created_at,
          _e.to_string()
        );
        //return Err(LemmyError::from_message(err_type));
        None
      }
    };

    // Update relate payment
    let mut payment_form = PiPaymentForm {
      person_id: None,
      ref_id: refid,
      testnet: settings.pinetwork.pi_testnet,
      finished: true,
      updated: Some(naive_now()),
      pi_uid: data.pi_uid,
      pi_username: "".to_string(), // data.pi_username.clone(), Hide username
      comment: data.comment.clone(),

      identifier: data.paymentid.clone(),
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
      PiPayment::update(conn, payment_id, &payment_form)
    })
    .await?
    {
      Ok(payment) => payment,
      Err(_e) => {
        let err_type = format!(
          "Register: Update payment complete error: {} {} {}",
          &data.info.username,
          &data.paymentid,
          _e.to_string()
        );
        //return Err(LemmyError::from_message(&err_type).into());
        return Ok(PiRegisterResponse {
          success: false,
          jwt: format!(""),
          extra: Some(format!("{}", err_type)),
        });
      }
    };

    // Persion is exist, change his password
    if change_password {
      let password_hash =
        hash(_new_password.clone(), DEFAULT_COST).expect("Couldn't hash password");

      let local_user_id;
      let _local_user = match blocking(context.pool(), move |conn| {
        LocalUserView::read_from_name(conn, &_new_user2)
      })
      .await?
      {
        Ok(lcu) => lcu,
        Err(_e) => {
          let err_type = format!(
            "Register: Update local user not found {} {} {}",
            &data.info.username,
            &data.paymentid,
            _e.to_string()
          );
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
        LocalUser::update_password(conn, local_user_id, &_new_password)
      })
      .await
      {
        Ok(chp) => chp,
        Err(_e) => {
          let err_type = format!(
            "Register: Update local user password error {} {} {}",
            &data.info.username,
            &data.paymentid,
            _e.to_string()
          );
          //return Err(LemmyError::from_message(&err_type).into());
          return Ok(PiRegisterResponse {
            success: false,
            jwt: format!(""),
            extra: Some(format!("{}", err_type)),
          });
        }
      };
      return Ok(PiRegisterResponse {
        success: true,
        jwt: Claims::jwt(
          local_user_id.0,
          &context.secret().jwt_secret,
          &context.settings().hostname,
        )?,
        extra: None,
      });
    }

    // We have to create both a person, and local_user

    // Register the new person
    let person_form = PersonInsertForm.builder()
      .name (data.info.username.to_owned())
      .actor_id( Some(actor_id.clone()))
      .private_key( Some(Some(actor_keypair.private_key)))
      .public_key( Some(actor_keypair.public_key))
      .inbox_url( Some(generate_inbox_url(&actor_id)?))
      .shared_inbox_url( Some(Some(generate_shared_inbox_url(&actor_id)?)))
      .admin( Some(no_admins))
      .external_id( Some(_pi_alias2))
      .build();

    // insert the person
    // let err_type = format!("user_already_exists: {} {}", &data.info.username, _pi_alias3);
    let inserted_person = match Person::create(conn, &person_form)
    .await
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

    //let default_listing_type = data.default_listing_type;
    //let default_sort_type = data.default_sort_type;

    //let inserted_person = inserted_person1.unwrap();
    let new_id = inserted_person.id.clone();
    // Create the local user
    let local_user_form = LocalUserForm {
      person_id: Some(new_id),
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
          Person::delete(conn, inserted_person.id)
        })
        .await??;

        return Err(LemmyError::from_message(err_type).into());
      }
    };

    let main_community_keypair = generate_actor_keypair()?;
    */
    // Return the jwt / LoginResponse?
    Ok(PiRegisterResponse {
      success: true,
      login: login_response,
      extra: None,
    })
    
    //Ok(login_response)
  }
}
