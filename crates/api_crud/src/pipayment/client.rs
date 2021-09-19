use crate::PerformCrud;
use actix_web::web::Data;
use lemmy_api_common::pipayment::*;
use lemmy_api_common::{blocking, person::*};
use lemmy_db_queries::{
  source::local_user::LocalUser_, source::pipayment::*, source::site::*, Crud, Followable,
  Joinable, ListingType, SortType,
};
use lemmy_db_schema::*;
use lemmy_db_schema::{
  source::{
    local_user::{LocalUser, LocalUserForm},
    pipayment::*,
    site::*,
  },
  PaymentId, PersonId, PiUserId,
};
use lemmy_utils::{
  claims::Claims,
  request::*,
  settings::structs::Settings,
  utils::{check_slurs, is_valid_actor_name},
  ApiError, ConnectionId, LemmyError,
};
use lemmy_websocket::{
  messages::{SendModRoomMessage, SendUserRoomMessage},
  LemmyContext, UserOperation,
};
use reqwest::Client;
use sha2::{Digest, Sha256, Sha512};
use uuid::Uuid;

pub async fn pi_payment(client: &Client, id: &str) -> Result<PiPaymentDto, LemmyError> {
  let fetch_url = format!("{}/payments/{}", Settings::get().pi_api_host(), id);

  let response = retry(|| {
    client
      .get(&fetch_url)
      .header("Authorization", format!("Key {}", Settings::get().pi_key()))
      .header("Content-Type", format!("application/json"))
      .send()
  })
  .await?;

  let res: PiPaymentDto = response
    .json::<PiPaymentDto>()
    .await
    .map_err(|e| RecvError(e.to_string()))?;
  Ok(res)
}

pub async fn pi_approve(client: &Client, id: &str) -> Result<PiPaymentDto, LemmyError> {
  let fetch_url = format!("{}/payments/{}/approve", Settings::get().pi_api_host(), id);

  let response = retry(|| {
    client
      .post(&fetch_url)
      .header("Authorization", format!("Key {}", Settings::get().pi_key()))
      .header("Content-Type", format!("application/json"))
      .send()
  })
  .await?;

  let res: PiPaymentDto = response
    .json::<PiPaymentDto>()
    .await
    .map_err(|e| RecvError(e.to_string()))?;
  Ok(res)
}

pub async fn pi_complete(
  client: &Client,
  id: &str,
  txid_: &str,
) -> Result<PiPaymentDto, LemmyError> {
  let fetch_url = format!("{}/payments/{}/complete", Settings::get().pi_api_host(), id);

  let r = TxRequest {
    txid: txid_.to_owned(),
  };
  let response = retry(|| {
    client
      .post(&fetch_url)
      .header("Authorization", format!("Key {}", Settings::get().pi_key()))
      .header("Content-Type", format!("application/json"))
      .json(&r)
      .send()
  })
  .await?;

  let res: PiPaymentDto = response
    .json::<PiPaymentDto>()
    .await
    .map_err(|e| RecvError(e.to_string()))?;
  Ok(res)
}

pub async fn pi_update_payment(
  context: &Data<LemmyContext>,
  approve: &PiApprove,
  tx: Option<String>,
) -> Result<PiPayment, LemmyError> {
  let payment_id = approve.paymentid.clone();
  let pi_username = approve.pi_username.clone();
  let pi_uid = approve.pi_uid.clone();
  let person_id = approve.person_id.clone();
  let comment = approve.comment.clone();
  
  // Hide PiUserName
  let mut sha256 = Sha256::new();
  sha256.update(pi_username.to_owned());
  let _pi_username: String = format!("{:X}", sha256.finalize());
  //let _pi_username = pi_username;

  let mut _payment_id = format!("{}", payment_id); //payment_id.to_string();
  let mut _payment_id2 = payment_id.to_string();
  //let _payment_id: String = "123".into();
  let _pi_uid = pi_uid;

  let mut approved = false;
  let mut completed = false;
  let mut exist = false;
  //let mut fetch_pi_server = true;
  let mut pid;
  let mut pmt;
  let mut _payment = match blocking(context.pool(), move |conn| {
    PiPayment::find_by_pipayment_id(&conn, &_payment_id.to_owned())
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

  let mut dto: Option<PiPaymentDto> = None;

  if _payment.is_some() {
    if !approved {
      let dto = match pi_approve(context.client(), &payment_id).await {
        Ok(c) => Some(c),
        Err(_e) => None,
      };
    } else if !completed {
      let dto = match pi_complete(context.client(), &payment_id, &tx.unwrap()).await {
        Ok(c) => Some(c),
        Err(_e) => None,
      };
    }
  } else {
    dto = match pi_approve(context.client(), &payment_id).await {
      Ok(c) => Some(c),
      Err(_e) => None,
    };
  }

  if !exist || !approved {
  } else {
  }

  // if _payment.is_some() {

  // }
  //let mut pm = None;

  let mut _payment_dto = PiPaymentDto {
    ..PiPaymentDto::default()
  };

  if dto.is_some() {
    _payment_dto = dto.unwrap();
  }

  // let refid = match info {
  //   Some(inf) => {
  //     let _info = info.unwrap();
  //     match _info.captcha_uuid {
  //       Some(uid) => match Uuid::parse_str(&uid) {
  //         Ok(uidx) => Some(uidx),
  //         Err(_e) => None
  //       },
  //       None => None
  //     }
  //   },
  //   None => {
  //     None
  //   }
  // };
  let refid = None;
  let create_at = match chrono::NaiveDateTime::parse_from_str(&_payment_dto.created_at, "%Y-%m-%dT%H:%M:%S%.f%Z"){
    Ok(dt) => Some(dt),
    Err(_e) => {
      let err_type = format!("Pi Server: get payment datetime error: user {}, paymentid {} {} {}", 
      &pi_username, &_payment_dto.identifier.clone(), _payment_dto.created_at, _e.to_string() );
      //return Err(ApiError::err(&err_type).into());  
      None
    }
  };

  let mut payment_form = PiPaymentForm {
    person_id: None,
    ref_id: refid,
    testnet: Settings::get().pi_testnet,
    finished: false,
    updated: None,
    pi_uid: _pi_uid,
    pi_username: _pi_username.clone(),
    comment: Some(_pi_username.clone()),

    identifier: _payment_dto.identifier.into(),
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
    metadata: None,
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
        pid = payment.id;
        Some(payment)
      }
      Err(_e) => {
        let err_type = _e.to_string();
        return Err(ApiError::err(&err_type).into());
      }
    };
    pmt = _payment.unwrap();
  } else {
    pmt = _payment.unwrap();
    pid = pmt.id;
    let inserted_payment = match blocking(context.pool(), move |conn| {
      PiPayment::update(&conn, pid, &payment_form)
    })
    .await?
    {
      Ok(payment) => Some(payment),
      Err(_e) => {
        let err_type = _e.to_string();
        return Err(ApiError::err(&err_type).into());
      }
    };
  }
  return Ok(pmt);
  //Ok(res)
}
