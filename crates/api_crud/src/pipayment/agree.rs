use crate::pipayment::client::*;
use crate::PerformCrud;
use actix_web::web::Data;
use lemmy_api_common::{blocking, password_length_check, pipayment::*};
use lemmy_db_queries::{
  source::person::*, source::pipayment::*, source::site::*, Crud,  
};
use lemmy_db_schema::source::{
  person::*,
  pipayment::*,
  site::*,
};
use lemmy_db_views_actor::person_view::PersonViewSafe;
use lemmy_utils::{
  settings::structs::Settings,
  utils::{check_slurs, is_valid_actor_name},
  ApiError, ConnectionId, LemmyError,
};
use lemmy_websocket::{messages::CheckCaptcha, LemmyContext};
use sha2::{Digest, Sha256};
//use chrono::*;
use uuid::Uuid;

#[async_trait::async_trait(?Send)]
impl PerformCrud for PiAgreeRegister {
  type Response = PiAgreeResponse;

  async fn perform(
    &self,
    context: &Data<LemmyContext>,
    _websocket_id: Option<ConnectionId>,
  ) -> Result<PiAgreeResponse, LemmyError> {
    let data: &PiAgreeRegister = self;

    // Make sure site has open registration
    if let Ok(site) = blocking(context.pool(), move |conn| Site::read_simple(conn)).await? {
      if !site.open_registration {
        return Err(ApiError::err("registration_closed").into());
      }
    }

    password_length_check(&data.info.password)?;

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

    if !is_valid_actor_name(&data.info.username) {
      return Err(ApiError::err("agree:invalid_username").into());
    }
    //check_slurs_opt(&data.paymentid.unwrap())?;
    //check_slurs_opt(&data.info.username)?;

    // Hide Pi user name, not store pi_uid
    let mut sha256 = Sha256::new();
    sha256.update(Settings::get().pi_seed());
    sha256.update(data.pi_username.to_owned());
    let _pi_username: String = format!("{:X}", sha256.finalize());
    let _pi_username2 = _pi_username.clone();
    //let _pi_username = data.pi_username.to_owned();

    let _payment_id = data.paymentid.to_owned();
    let _new_user = data.info.username.to_owned();
    let _pi_uid = data.pi_uid.clone();

    let mut approved = false;
    let mut completed = false;
    let mut exist = false;
    //let mut fetch_pi_server = true;
    let mut pid;
    let mut dto: Option<PiPaymentDto> = None;

    let mut _payment = match blocking(context.pool(), move |conn| {
      PiPayment::find_by_pipayment_id(&conn, &_payment_id)
    })
    .await?
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
      return Err(ApiError::err(&err_type).into());
      // if (!approved) {
      //   let dto = match pi_approve(context.client(), &data.paymentid.clone())
      //   .await
      //   {
      //     Ok(c) => Some(c),
      //     Err(_e) => None,
      //   };
      // }
    } else {
    }

    let mut pi_person = match blocking(context.pool(), move |conn| {
      Person::find_by_pi_name(&conn, &_pi_username)
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
    match pi_person {
      Some(pi) => {
        match person {
          Some(per) => {
            if pi.name != per.name {
              let err_type = format!(
                "User {} is exist and belong other Pi account",
                &data.info.username
              );
              return Err(ApiError::err(&err_type).into());
            } else {
              // Same name and account: change password ???
            }
          }
          None => {
            // Not allow change username
            let err_type = format!("Account already have user name {}", pi.name);
            return Err(ApiError::err(&err_type).into());
          }
        };
      }
      None => {
        match person {
          Some(per) => {
            let err_type = format!("User {} is exist", &data.info.username);
            return Err(ApiError::err(&err_type).into());
          }
          None => {
            // No account, we approved this tx
          }
        };
      }
    }
    
    dto = match pi_approve(context.client(), &data.paymentid.clone()).await {
      Ok(c) => Some(c),
      Err(_e) => {
        // Pi Server error
        let err_type = format!("Pi Server Error: approve user {}, paymentid {}, error: {}", &data.info.username,  &data.paymentid, _e.to_string());
        //let err_type = _e.to_string();
        return Err(ApiError::err(&err_type).into());
      }
    };
    
    let mut _payment_dto = PiPaymentDto {
      ..PiPaymentDto::default()
    };
    _payment_dto.status.developer_approved  =  true;

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
        let err_type = format!("Pi Server Error: get payment datetime error: user {}, paymentid {} {} {}", 
        &data.info.username, &data.paymentid, _payment_dto.created_at, _e.to_string() );
        return Err(ApiError::err(&err_type).into());  
      }
    };

    let mut payment_form = PiPaymentForm {
      person_id: None,
      ref_id: refid,
      testnet: Settings::get().pi_testnet,
      finished: false,
      updated: None,
      pi_uid: data.pi_uid,
      pi_username: "".to_string(), //data.pi_username.clone(), => Hide user info
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

    //if !exist {
      _payment = match blocking(context.pool(), move |conn| {
        PiPayment::create(&conn, &payment_form)
      })
      .await?
      {
        Ok(payment) => {
            pid = payment.id;
            Some(payment)
        },
        Err(_e) => {
          // let err_type = if e.to_string() == "value too long for type character varying(200)" {
          //   "post_title_too_long"
          // } else {
          //   "couldnt_create_post"
          // };
          let err_type = format!("Error insert payment for agree: user {}, paymentid {} error: {}", &data.info.username,  &data.paymentid, _e.to_string());
          return Err(ApiError::err(&err_type).into());
        }
      };
      /*
      //pid = _payment.unwrap().id;
    } else {
      let pmt = _payment.unwrap();
      pid = pmt.id;
      let _payment = match blocking(context.pool(), move |conn| {
        PiPayment::update(&conn, pid, &payment_form)
      })
      .await?
      {
        Ok(payment) => payment,
        Err(_e) => {
          // let err_type = if e.to_string() == "value too long for type character varying(200)" {
          //   "post_title_too_long"
          // } else {
          //   "couldnt_create_post"
          // };
          let err_type = e.to_string();
          return Err(ApiError::err(&err_type).into());
        }
      };
    }
    */
    //_payment = pi_update_payment(context, payment_id, &_pi_username, _pi_uid, Some(data.info)).await?;
    // Return the jwt
    Ok(PiAgreeResponse {
      id: pid,
      //id: _payment.map(|x| x.id),
      paymentid: data.paymentid.to_owned(),
      payment: None, //_payment,
      extra: Some(_pi_username2),
    })
  }
}
