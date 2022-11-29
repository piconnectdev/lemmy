use crate::pipayment::client::*;
use crate::PerformCrud;
use actix_web::web::Data;
use lemmy_api_common::{
  pipayment::*,
  utils::{password_length_check},
};
use lemmy_db_schema::{
  //impls::pipayment::PiPaymentModerator,
  newtypes::*,
  source::{person::*, pipayment::*, site::*},
  traits::Crud,
};
use lemmy_db_views_actor::structs::PersonViewSafe;
use lemmy_utils::{
  error::LemmyError,
  settings::SETTINGS,
  utils::{check_slurs, is_valid_actor_name},
  ConnectionId,
};
use lemmy_db_views::structs::SiteView;
use lemmy_websocket::{messages::CheckCaptcha, LemmyContext};
use sha2::{Digest, Sha256};
use uuid::Uuid;

#[async_trait::async_trait(?Send)]
impl PerformCrud for PiAgreeRegister {
  type Response = PiAgreeResponse;

  async fn perform(
    &self,
    context: &Data<LemmyContext>,
    _websocket_id: Option<ConnectionId>,
  ) -> Result<PiAgreeResponse, LemmyError> {
    let settings = SETTINGS.to_owned();
    let data: &PiAgreeRegister = self;

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

    let mut result_string = "".to_string();
    let mut result = true;
    let mut completed = false;

    password_length_check(&data.info.password)?;

    // Check if there are admins. False if admins exist
    // let no_admins = blocking(context.pool(), move |conn| {
    //   PersonViewSafe::admins(conn).map(|a| a.is_empty())
    // })
    // .await??;
    //let no_admins = PersonViewSafe::admins(context.pool()).await.map(|a| a.is_empty()).unwrap();

    // If its not the admin, check the captcha
    // if local_site.site_setup && local_site.captcha_enabled {
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

    // check_slurs(&data.info.username, &context.settings().slur_regex())?;

    // if !is_valid_actor_name(
    //   &data.info.username,
    //   context.settings().actor_name_max_length,
    // ) {
    //   println!("Invalid username {}", &data.info.username);
    //   return Err(LemmyError::from_message("agree:invalid_username"));
    // }
    //check_slurs_opt(&data.paymentid.unwrap(), &context.settings().slur_regex())?;
    //check_slurs_opt(&data.info.username, &context.settings().slur_regex())?;

    // Hide Pi user name, not store pi_uid
    // let mut sha256 = Sha256::new();
    // sha256.update(settings.pi_seed());
    // sha256.update(data.pi_username.to_owned());
    // let _pi_alias: String = format!("{:X}", sha256.finalize());
    // let _pi_alias2 = _pi_alias.clone();
    let _pi_alias = data.ea.account.to_owned();

    let _payment_id = data.paymentid.to_owned();
    let _pi_uid = data.ea.extra.clone();
    let _new_user = data.info.username.to_owned();

    let mut approved = false;
    let mut completed = false;
    let mut exist = false;
    //let mut fetch_pi_server = true;
    let mut pid;
    let mut dto: Option<PiPaymentDto> = None;

    let mut _payment = match PiPayment::find_by_pipayment_id(context.pool(), &_payment_id).await
    {
      Ok(c) => {
        exist = true;
        approved = c.approved;
        completed = c.completed;
        pid = c.id;
        Some(c)
      }
      Err(_e) => None,
    };

    if _payment.is_some() {
      // Why here ????
      let err_type = format!("Payment {} was approved", data.paymentid);
      return Ok(PiAgreeResponse {
        success: true,
        id: None,
        paymentid: data.paymentid.to_owned(),
        extra: None,
      });
    }

    let other_person = match Person::find_by_extra_name(context.pool(), &_pi_alias).await
    {
      Ok(c) => Some(c),
      Err(_e) => None,
    };

    let person = match Person::find_by_name(context.pool(), &_new_user).await
    {
      Ok(c) => Some(c),
      Err(_e) => None,
    };

    match other_person {
      Some(pi) => {
        match person {
          Some(per) => {
            if pi.external_id != per.external_id {
              let err_type = format!(
                "User {} is exist and belong to other Pi Network account",
                &data.info.username
              );
              //println!("{} {} {}", data.pi_username.clone(), err_type, &_pi_alias2);
              result_string = err_type.clone();
              result = false
            } else {
              // Same name and account: change password ???
              result = true;
            }
          }
          None => {
            // Not allow change username ???
            let err_type = format!("Your account already exist: {}", pi.name);
            println!("{} {} {}", data.ea.account.clone(), err_type, &_pi_alias.clone());
            result_string = err_type.clone();
            result = false;
          }
        };
      }
      None => {
        match person {
          Some(per) => {
            let err_type = format!(
              "User {} is exist, create same user name is not allow!",
              &data.info.username
            );
            println!("{} {} {}", data.ea.account.clone(), err_type, &_pi_alias.clone());
            result_string = err_type.clone();
            result = false;
          }
          None => {
            // No account, we approved this tx
            result = true;
          }
        };
      }
    }

    dto = match pi_approve(context.client(), &data.paymentid.clone()).await {
      Ok(c) => Some(c),
      Err(_e) => {
        // Pi Server error
        let err_type = format!(
          "Pi Server Error: approve user {}, paymentid {}, error: {}",
          &data.info.username,
          &data.paymentid,
          _e.to_string()
        );
        //let err_type = _e.to_string();
        return Err(LemmyError::from_message(&err_type));
      }
    };

    let mut _payment_dto = PiPaymentDto {
      ..PiPaymentDto::default()
    };
    _payment_dto.status.developer_approved = true;

    if dto.is_some() {
      _payment_dto = dto.unwrap();
    }

    // TODO: UUID check
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
          "Pi Server Error: get payment datetime error: user {}, paymentid {} {}",
          &data.info.username, &data.paymentid, _payment_dto.created_at
        );
        //return Err(LemmyError::from_message((&err_type));
        None
      }
    };

    let mut payment_form = PiPaymentInsertForm::builder()
      .person_id( None)
      .ref_id( refid)
      .testnet( settings.pinetwork.pi_testnet)
      .finished( false)
      .updated( None)
      //.pi_uid( data.pi_uid)
      .pi_uid( None)
      .pi_username( "".to_string()) //data.pi_username.clone(), => Hide user info
      .comment( data.ea.comment.clone()) // Peer address

      .identifier( data.paymentid.clone())
      .user_uid( _payment_dto.user_uid)
      .amount( _payment_dto.amount)
      .memo( _payment_dto.memo)
      .to_address( _payment_dto.to_address) // Site's own address
      .created_at( create_at)
      .approved( _payment_dto.status.developer_approved)
      .verified( _payment_dto.status.transaction_verified)
      .completed( _payment_dto.status.developer_completed)
      .cancelled( _payment_dto.status.cancelled)
      .user_cancelled( _payment_dto.status.user_cancelled)
      .tx_link( "".to_string())
      .tx_id( "".to_string())
      .tx_verified( false)
      .metadata( _payment_dto.metadata)
      .extras( None)
      .build();

    match _payment_dto.transaction {
      Some(tx) => {
        payment_form.tx_link = tx._link;
        payment_form.tx_verified = tx.verified;
        payment_form.tx_id = tx.txid;
      }
      None => {}
    }

    //if !exist {
    _payment = match PiPayment::create(context.pool(), &payment_form).await
    {
      Ok(payment) => {
        pid = payment.id;
        Some(payment)
      }
      Err(_e) => {
        let err_type = format!(
          "Error insert payment for agree: user {}, paymentid {} error: {}",
          &data.info.username,
          &data.paymentid,
          _e.to_string()
        );
        return Err(LemmyError::from_message(&err_type));
      }
    };
    Ok(PiAgreeResponse {
      success: result,
      id: Some(pid),
      paymentid: data.paymentid.to_owned(),
      extra: Some(result_string),
    })
  }
}
