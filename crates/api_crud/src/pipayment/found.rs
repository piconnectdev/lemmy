use crate::pipayment::client::*;
use crate::PerformCrud;
use actix_web::web::Data;
use lemmy_api_common::{blocking, password_length_check, person::*, pipayment::*};
use lemmy_apub::{
  generate_apub_endpoint, generate_followers_url, generate_inbox_url, generate_shared_inbox_url,
  EndpointType,
};
use lemmy_db_queries::{
  source::local_user::LocalUser_, source::pipayment::*, source::site::*, Crud, Followable, Joinable, ListingType,
  SortType,
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
  PaymentId
};
//use lemmy_db_views_actor::person_view::PersonViewSafe;
use lemmy_utils::{
  apub::generate_actor_keypair,
  claims::Claims,
  request::*,
  settings::structs::Settings,
  utils::{check_slurs, is_valid_actor_name},
  ApiError, ConnectionId, LemmyError,
};
use lemmy_websocket::{messages::CheckCaptcha, LemmyContext};
use sha2::{Digest, Sha256, Sha512};
use uuid::Uuid;

#[async_trait::async_trait(?Send)]
impl PerformCrud for PiPaymentFound {
  type Response = PiPaymentFoundResponse;

  async fn perform(
    &self,
    context: &Data<LemmyContext>,
    _websocket_id: Option<ConnectionId>,
  ) -> Result<PiPaymentFoundResponse, LemmyError> {
    let data: &PiPaymentFound = self;

    //check_slurs(&data.pi_username)?;
    //check_slurs_opt(&data.paymentid.unwrap())?;
    //check_slurs_opt(&data.username)?;
    let mut sha256 = Sha256::new();
    sha256.update(Settings::get().pi_seed());
    sha256.update(data.pi_username.to_owned());
    let _pi_username: String = format!("{:X}", sha256.finalize());
    let _pi_username2 = _pi_username.clone();

    let _payment_id = data.paymentid.to_owned();
    let _pi_username = data.pi_username.to_owned();
    let _pi_uid = data.pi_uid.clone();

    let mut approved = false;
    let mut completed = false;
    let mut finished = false;
    let mut cancelled = false;
    let mut exist = false;
    let mut payment;
    let mut txid: String;
    let mut payment_id: PaymentId;
    let mut dto: Option<PiPaymentDto> = None;
    let mut updated: Option<chrono::NaiveDateTime> = None;
    let mut _payment = match blocking(context.pool(), move |conn| {
      PiPayment::find_by_pipayment_id(&conn, &_payment_id)
    })
    .await?
    {
      Ok(c) => {
        approved = c.approved;
        completed = c.completed;
        finished = c.finished;
        cancelled = c.cancelled;
        payment_id = c.id;
        payment_id = c.id;
        //old_comment = c.comment; 
        Some(c)
      },
      Err(_e) => None,
    };

    if _payment.is_some() {
      exist = true;
      updated = Some(naive_now());
      //payment = _payment.unwrap();
      //payment_id = payment.id;
      //txid = payment.tx_id.clone();
    } else {
      exist = false;
    }

    let dto_read = pi_payment(context.client(), &data.paymentid.clone()).await?;
      //   let dto_read = match pi_payment(context.client(), &data.paymentid.clone()).await{
      //   Ok(c) => Some(c),
      //   Err(_e) => {
      //     // Pi Server error
      //     let err_type = format!("Pi Server: error while check payment: user {}, paymentid {} error: {}", &data.pi_username,  &data.paymentid, _e.to_string());
      //     return Err(ApiError::err(&err_type).into());
      //   }
      // };
      approved = dto_read.status.developer_approved;
      completed = dto_read.status.developer_completed;
      cancelled = dto_read.status.cancelled;      
      let mut tx;
      if cancelled {
        let err_type = format!("Pi Server: payment cancelled: user {}, paymentid {}", &data.pi_username, &data.paymentid);
        return Err(ApiError::err(&err_type).into());
      }

      if !approved {
        dto = match pi_approve(context.client(), &data.paymentid.clone()).await {
          Ok(c) => {
            Some(c)
          },
          Err(_e) => {
            // Pi Server error
            let err_type = format!("Pi Server: Error while approve: user {}, paymentid {} error: {}", &data.pi_username, &data.paymentid, _e.to_string());
            //let err_type = _e.to_string();
            return Err(ApiError::err(&err_type).into());
          }
        };
      } else {
        if !completed {
          tx = dto_read.transaction.clone();
          match tx {
              Some(tx_) => {
              //let err_type = format!("Pi Server: Error while complete: user {}, paymentid {}", &data.pi_username, &data.paymentid);
              //return Err(ApiError::err(&err_type).into());
              txid = tx_.txid;
              dto = match pi_complete(context.client(), &data.paymentid.clone(), &txid.clone() ).await {
                Ok(c) => {
                  Some(c)
                },
                Err(_e) => {
                  // Pi Server error
                  let err_type = format!("Pi Server: Error while completed: user {}, paymentid {} error: {}", &data.pi_username,  &data.paymentid, _e.to_string());
                  return Err(ApiError::err(&err_type).into());
                }
              };
            },
            None => {
              let err_type = format!("Pi Server: Error while completed, no transaction: user {}, paymentid {}", &data.pi_username,  &data.paymentid);
              return Err(ApiError::err(&err_type).into());
            }
          };
        };
        finished = true;         
      } 

    let mut _payment_dto = PiPaymentDto {
      ..PiPaymentDto::default()
    };
    _payment_dto.status.developer_approved  =  true;

    if dto.is_some() {
      _payment_dto = dto.unwrap();
      //tx = _payment_dto.transaction;
    } else {
      _payment_dto = dto_read;
    }

    // let refid = match &data.info.captcha_uuid {
    //   Some(uid) => match Uuid::parse_str(uid) {
    //     Ok(uidx) => Some(uidx),
    //     Err(_e) => None,
    //   },
    //   None => None,
    // };

    let create_at = match chrono::NaiveDateTime::parse_from_str(&_payment_dto.created_at, "%Y-%m-%dT%H:%M:%S%.f%Z"){
      Ok(dt) => Some(dt),
      Err(_e) => {
        let err_type = format!("Pi Server: get payment datetime error: user {}, paymentid {} {} {}", 
        &data.pi_username, &data.paymentid, _payment_dto.created_at, _e.to_string() );
        //return Err(ApiError::err(&err_type).into());  
        None
      }
    };

    let mut payment_form = PiPaymentForm {
      person_id: None,
      ref_id: None,
      testnet: Settings::get().pi_testnet,
      finished: finished,
      updated: updated,
      pi_uid: data.pi_uid, //data.pi_uid
      pi_username: "".to_string(), //data.pi_username.clone(), Hide user name
      comment: None, //"".to_string(),

      identifier: data.paymentid.clone(),
      user_uid: _payment_dto.user_uid.clone(), //"".to_string(), //_payment_dto.user_uid,
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

    if !exist {
      _payment = match blocking(context.pool(), move |conn| {
        PiPayment::create(&conn, &payment_form)
      })
      .await?
      {
        Ok(payment) => {
            payment_id = payment.id;
            Some(payment)
        },
        Err(_e) => {
          // let err_type = if e.to_string() == "value too long for type character varying(200)" {
          //   "post_title_too_long"
          // } else {
          //   "couldnt_create_post"
          // };
          let err_type = format!("Error insert payment: user {}, paymentid {} error: {}", &data.pi_username,  &data.paymentid, _e.to_string());

          return Err(ApiError::err(&err_type).into());
        }
      };
    } else {
      payment = _payment.unwrap();
      payment_id = payment.id;
      _payment = match blocking(context.pool(), move |conn| {
        PiPayment::update(&conn, payment_id, &payment_form)
      })
      .await?
      {
        Ok(payment) => Some(payment),
        Err(_e) => {
          // let err_type = if e.to_string() == "value too long for type character varying(200)" {
          //   "post_title_too_long"
          // } else {
          //   "couldnt_create_post"
          // };
          let err_type = format!("Error update payment: user {}, paymentid {} error: {}", &data.pi_username,  &data.paymentid, _e.to_string());
          return Err(ApiError::err(&err_type).into());
        }
      };
    }
    let payment = _payment.unwrap();
    Ok(PiPaymentFoundResponse {
      id: payment.id,
      paymentid: payment.identifier,
    })
  }
}
