use crate::PerformCrud;
use lemmy_api_common::pipayment::*;
use lemmy_api_common::{blocking, password_length_check, person::*, pipayment::*};
use lemmy_db_schema::*;
use lemmy_db_queries::{
  source::local_user::LocalUser_, source::site::*, source::pipayment::*, Crud, Followable, Joinable, ListingType,
  SortType,
};
use lemmy_db_schema::{source::{
  community::*,
  local_user::{LocalUser, LocalUserForm},
  person::*,
  pipayment::*,
  site::*,
}, PaymentId, PiPaymentId, PiUserId, PersonId, };
use lemmy_utils::{
  apub::generate_actor_keypair,
  claims::Claims,
  request::*,
  settings::structs::Settings,
  utils::{check_slurs, is_valid_username},
  ApiError, ConnectionId, LemmyError,
};
use lemmy_websocket::{
  messages::{SendModRoomMessage, SendUserRoomMessage},
  LemmyContext,
  UserOperation,
};
use actix_web::web::Data;
use reqwest::Client;
use uuid::Uuid;

pub async fn pi_payment(client: &Client, id: &PiPaymentId) -> Result<PiPaymentDto, LemmyError> {
  let fetch_url = format!("{}/payments/{}", Settings::get().pi_api_host(), id);

  let response = retry(|| {
    client
      .get(&fetch_url)
      .header("Authorization", format!("Key {}", Settings::get().pi_key()))
      .send()
  })
  .await?;

  let res: PiPaymentDto = response
    .json::<PiPaymentDto>()
    .await
    .map_err(|e| RecvError(e.to_string()))?;
  Ok(res)
}

pub async fn pi_approve(client: &Client, id: &PiPaymentId) -> Result<PiPaymentDto, LemmyError> {
  let fetch_url = format!("{}/payments/{}/approve", Settings::get().pi_api_host(), id);

  let response = retry(|| {
    client
      .post(&fetch_url)
      .header("Authorization", format!("Key {}", Settings::get().pi_key()))
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
  id: &String,
  txid_: &String,
) -> Result<PiPaymentDto, LemmyError> {
  let fetch_url = format!("{}/payments/{}/complete", Settings::get().pi_api_host(), id);

  let r = TxRequest {
    txid: txid_.to_owned(),
  };
  let response = retry(|| {
    client
      .post(&fetch_url)
      .header("Authorization", format!("Key {}", Settings::get().pi_key()))
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
  payment_id: PiPaymentId,
  pi_username: &String,
  pi_uid: Option<PiUserId>,
  info: Option<Register>
) -> Result<PiPayment, LemmyError> {
  
  let _payment_id = payment_id;
  let _pi_username = pi_username;
  let _pi_uid = pi_uid;

  let mut approved = false;
    let mut completed = false;
    let mut exist = false;
    //let mut fetch_pi_server = true;
    let mut pid;
    let mut pmt;
    let mut _payment = match blocking(context.pool(), move |conn| {
      PiPayment::find_by_pipayment_id(&conn, _payment_id)
    })
    .await?
    {
      Ok(c) => {
        exist = true;
        approved = c.approved;
        completed = c.completed;
        pid = c.id;
        Some(c)
      },
      Err(_e) => {
        None
      },
    };

    let mut dto: Option<PiPaymentDto> = None;

    if (_payment.is_some()) {
      if (!approved) {
        let dto = match pi_approve(context.client(), &_payment_id.clone())
        .await
        {
          Ok(c) => Some(c),
          Err(_e) => None,
        };
      }
    } else {
      dto = match pi_approve(context.client(), &_payment_id.clone())
      .await
      {
        Ok(c) => Some(c),
        Err(_e) => None,
      };
    }

    if !exist || ! approved {
      
    } else {

    }


    // if _payment.is_some() {

    // }
    //let mut pm = None;

    let mut _payment_dto = PiPaymentDto{
      ..PiPaymentDto::default()
    };

    if dto.is_some() {
      _payment_dto = dto.unwrap();
    }

    let _info = info.unwrap();
    let refid = match _info.captcha_uuid {
      Some(uid) =>  match Uuid::parse_str(&uid) {
        Ok(uidx) => Some(uidx),
        Err(_e) => None,
      }
      None => None,
    };

    let mut payment_form = PiPaymentForm {
      person_id: None,
      ref_id: refid,
      testnet: Settings::get().pi_testnet() ,
      finished: false,
      updated: None,
      pi_payment_id: _payment_id,
      pi_uid: _pi_uid,
      pi_username: _pi_username.clone(),
      comment: Some(_pi_username.clone()),

      identifier: _payment_dto.identifier,
      user_uid: _payment_dto.user_uid,
      amount: _payment_dto.amount,
      memo: _payment_dto.memo,
      to_address: _payment_dto.to_address,
      created_at: _payment_dto.created_at,
      approved: _payment_dto.status.developer_approved,
      verified: _payment_dto.status.transaction_verified,
      completed: _payment_dto.status.developer_completed,
      cancelled: _payment_dto.status.cancelled,
      user_cancelled: _payment_dto.status.user_cancelled,
      tx_link: "".to_string(),
      tx_id: "".to_string(),
      tx_verified: false,
      //tx_id:  _payment_dto.transaction.map(|tx| tx.txid),
      //payment_dto: _payment_dto,
      //..PiPaymentForm::default()
      metadata: None,
      //dto: None,
    };

    match _payment_dto.transaction {
      Some(tx) =>  {
        payment_form.tx_link = tx._link; 
        payment_form.tx_verified = tx.verified;
        payment_form.tx_id = tx.txid;          
      },
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
      },
      Err(e) => {
        // let err_type = if e.to_string() == "value too long for type character varying(200)" {
        //   "post_title_too_long"
        // } else {
        //   "couldnt_create_post"
        // };
        let err_type = e.to_string();
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
      Err(e) => {
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
  return Ok(pmt);
  //Ok(res)
}
