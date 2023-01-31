use chrono::Duration;

use actix_web::web::Data;
use lemmy_api_common::pipayment::*;
use lemmy_db_schema::{
  newtypes::{CommentId, *},
  source::{comment::*, pipayment::*, post::*, person::*},
  traits::{Crud, Signable } ,
  utils::naive_now, schema::pipayment::user_cancelled,
};
use lemmy_utils::{error::LemmyError, request::retry, settings::SETTINGS, REQWEST_TIMEOUT};
use lemmy_api_common::{context::LemmyContext, websocket::structs::PiTokenItem};
use reqwest_middleware::{ ClientWithMiddleware};

use sha2::{Digest, Sha256};
use uuid::Uuid;

pub fn hide_username(name: &str) -> String {
  let settings = SETTINGS.to_owned();
  let mut sha256 = Sha256::new();
  sha256.update(settings.pi_seed());
  sha256.update(name.clone().to_owned());
  let username: String = format!("{:X}", sha256.finalize());
  return username;
}

pub async fn pi_payment(
  client: &ClientWithMiddleware, 
  id: &str,
) -> Result<PiPaymentDto, LemmyError> {
  let settings = SETTINGS.to_owned();
  let fetch_url = format!("{}/payments/{}", settings.pi_api_host(), id);

  let response = retry(|| {
    client
      .get(&fetch_url)
      .timeout(REQWEST_TIMEOUT)
      .header("Authorization", format!("Key {}", settings.pi_key()))
      .header("Content-Type", format!("application/json"))
      .send()
  })
  .await?;

  let res: PiPaymentDto = response
    .json::<PiPaymentDto>()
    .await
    .map_err(|e| LemmyError::from_error_message(e, ""))?;
  //.map_err(|e| RecvError(e.to_string()))?;
  Ok(res)
}

pub async fn pi_incompleted_server_payments(
  client: &ClientWithMiddleware, 
) -> Result<Vec<PiPaymentDto>, LemmyError> {
  let settings = SETTINGS.to_owned();
  let fetch_url = format!("{}/payments/incomplete_server_payments", settings.pi_api_host());

  let response = retry(|| {
    client
      .get(&fetch_url)
      .timeout(REQWEST_TIMEOUT)
      .header("Authorization", format!("Key {}", settings.pi_key()))
      .header("Content-Type", format!("application/json"))
      .send()
  })
  .await?;

  let res: Vec<PiPaymentDto> = response
    .json::<Vec<PiPaymentDto>>()
    .await
    .map_err(|e| LemmyError::from_error_message(e, ""))?;
  Ok(res)
}

pub async fn pi_approve(
  client: &ClientWithMiddleware,
  id: &str,
) -> Result<PiPaymentDto, LemmyError> {
  let settings = SETTINGS.to_owned();
  let fetch_url = format!("{}/payments/{}/approve", settings.pi_api_host(), id);

  let response = retry(|| {
    client
      .post(&fetch_url)
      .header("Authorization", format!("Key {}", settings.pi_key()))
      .header("Content-Type", format!("application/json"))
      .send()
  })
  .await?;

  let res: PiPaymentDto = response
    .json::<PiPaymentDto>()
    .await
    .map_err(|e| LemmyError::from_error_message(e, ""))?;
  Ok(res)
}


pub async fn pi_create(
  client: &ClientWithMiddleware,
  payment: &PiPaymentArgs,
) -> Result<PiPaymentDto, LemmyError> {
  let settings = SETTINGS.to_owned();
  let fetch_url = format!("{}/payments", settings.pi_api_host());

  let response = retry(|| {
    client
      .post(&fetch_url)
      .header("Authorization", format!("Key {}", settings.pi_key()))
      .header("Content-Type", format!("application/json"))
      .json(&payment)
      .send()
  })
  .await?;

  let res: PiPaymentDto = response
    .json::<PiPaymentDto>()
    .await
    .map_err(|e| LemmyError::from_error_message(e, "Can not create A2U payment"))?;
  Ok(res)
}

pub async fn pi_cancel(
  client: &ClientWithMiddleware,
  id: &str,
) -> Result<PiPaymentDto, LemmyError> {
  let settings = SETTINGS.to_owned();
  let fetch_url = format!("{}/payments/{}/cancel", settings.pi_api_host(), id);

  let response = retry(|| {
    client
      .post(&fetch_url)
      .header("Authorization", format!("Key {}", settings.pi_key()))
      .header("Content-Type", format!("application/json"))
      .send()
  })
  .await?;

  let res: PiPaymentDto = response
    .json::<PiPaymentDto>()
    .await
    .map_err(|e| LemmyError::from_error_message(e, ""))?;
  Ok(res)
}

pub async fn pi_complete(
  client: &ClientWithMiddleware,
  id: &str,
  txid_: &str,
) -> Result<PiPaymentDto, LemmyError> {
  let settings = SETTINGS.to_owned();
  let fetch_url = format!("{}/payments/{}/complete", settings.pi_api_host(), id);

  let r = TxRequest {
    txid: txid_.to_owned(),
  };

  let response = retry(|| {
    client
      .post(&fetch_url)
      .header("Authorization", format!("Key {}", settings.pi_key()))
      .header("Content-Type", format!("application/json"))
      .json(&r)
      .send()
  })
  .await?;

  let res: PiPaymentDto = response
    .json::<PiPaymentDto>()
    .await
    .map_err(|e| LemmyError::from_error_message(e, ""))?;
  Ok(res)
}


pub async fn pi_me(context: &Data<LemmyContext>, key: &str) -> Result<PiUserDto, LemmyError> {
  let settings = SETTINGS.to_owned();
  let fetch_url = format!("{}/me", settings.pi_api_host());
  let client = context.client();
   match context.chat_server().check_pi_token(key.to_string().clone(), "".to_string())?
        {
          Some(p) => {
            return Ok(p)
          },
          None => {
          }
        }
      
  let response = retry(|| {
    client
      .get(&fetch_url)
      .header("Authorization", format!("Bearer {}", &key.clone()))
      .header("Content-Type", format!("application/json"))
      .send()
  })
  .await?;

  let res: PiUserDto = response
    .json::<PiUserDto>()
    .await
    .map_err(|e| LemmyError::from_error_message(e, "Fetch /me error"))?;
    
  let token_item = PiTokenItem {
    answer: res.clone(),
    uuid: key.to_string(),
    expires: naive_now() + Duration::days(3), // expires in 5 days
  };
  // Stores the PiTokenItem item on the queue
  context.chat_server().add_pi_token(token_item);

  Ok(res)
}

pub async fn pi_payment_update(
  context: &Data<LemmyContext>,
  approve: &PiApprove,
  pipayment: Option<PiPayment>,
  tx: Option<String>,
) -> Result<PiPayment, LemmyError> {

  let pi_username = approve.pi_username.clone();
  let pi_uid = approve.pi_uid.clone();
  let payment_id = approve.paymentid.clone();  
  //let comment = approve.comment.clone();
  let comment = approve.comment.clone().unwrap_or("".to_string());
  let mut person_id: Option<PersonId> = None;
  let mut verified = false;
  let person = match Person::find_by_extra_name(context.pool(), &pi_username.clone()).await
  {
    Ok(c) => {
      person_id = Some(c.id.clone());
      verified = c.verified;
      Some(c)
    },
    Err(_e) => None
  };

  let _pi_user_alias = pi_username;

  let mut _payment_id = format!("{}", payment_id);
  let _pi_uid = pi_uid;

  let mut exist = false;
  let mut fetch_pi_server = false;
  let mut approved = false;
  let mut completed = false;
  let mut finished = false;
  let mut cancelled = false;
  let mut usercancelled = false;
  let mut txid: String = "".to_string();
  let mut txlink: String = "".to_string();
  let mut dto: Option<PiPaymentDto> = None;

  let mut pid;
  let mut pmt;
  // let mut _payment = match PiPayment::find_by_pipayment_id(context.pool(), &_payment_id.clone().to_owned()).await
  // {
  //   Ok(c) => {
  //     exist = true;
  //     approved = c.approved;
  //     completed = c.completed;
  //     cancelled = c.cancelled;
  //     txid = c.tx_id.clone();
  //     txlink = c.tx_link.clone();
  //     pid = c.id;
  //     Some(c)
  //   }
  //   Err(_e) => None,
  // };
  if pipayment.is_some() {
    let c = pipayment.clone().unwrap();
    exist = true;
    approved = c.approved;
    completed = c.completed;
    cancelled = c.cancelled;
    txid = c.tx_id.clone();
    txlink = c.tx_link.clone();
    pid = c.id;
    if cancelled || completed {
      finished = true;
      fetch_pi_server = false;
    } else {
      fetch_pi_server = true;
    }
  } else {
    fetch_pi_server = true;
  }

  if fetch_pi_server && !finished {
    dto = match pi_payment(context.client(), &_payment_id.clone()).await {
      Ok(c) => {
        approved = c.status.developer_approved;
        completed = c.status.developer_completed;
        cancelled = c.status.cancelled;
        usercancelled = c.status.user_cancelled;
        if c.transaction.is_some() {
          txid = c.transaction.clone().unwrap().txid;
        }
        println!("pi_payment_update, fetch payment from server: {} - {} approved:{} completed:{} cancelled:{} user_cancelled:{} {}", _pi_user_alias.clone(), _payment_id.clone(), approved, completed, cancelled, usercancelled, txid.clone());
        Some(c)
      }
      Err(_e) => {
        // Pi Server error
        let err_str = format!(
          "Pi Server: error while check payment: user {}, paymentid {} error: {}",
          _pi_user_alias.clone(),
          _payment_id.clone(),
          _e.to_string()
        );
        println!("pi_payment_update, load payment from server error: {} - {} approved:{} completed:{} cancelled:{} user_cancelled:{}", _pi_user_alias.clone(), _payment_id.clone(), approved, completed, cancelled, usercancelled);
        return Err(LemmyError::from_message(&err_str));
      }
    };
  }

  if !approved {
    if pipayment.is_some() {
      let err_str = format!(
        "The payment: user {}, paymentid {} was approved",
        _pi_user_alias.clone(),
        _payment_id.clone()
      );
      return Err(LemmyError::from_message(&err_str));
    } 
    dto = match pi_approve(context.client(), &payment_id).await {
      Ok(c) => { 
        println!("pi_payment_update, pi_approve return dto: {} {} {}", _payment_id.clone(), c.amount, c.memo.clone());
        Some(c)
      },
      Err(_e) => {
        let err_str = format!(
          "Pi Server: approve payment error {}, paymentid {} error: {}",
          _pi_user_alias.clone(),
          _payment_id.clone(),
          _e.to_string()
        );
        println!("pi_payment_update, {}", err_str.clone());
        return Err(LemmyError::from_message(&err_str));
      },
    };
  } else if !completed {
    if tx.is_some() {
      txid = tx.unwrap();
    }
    println!("pi_payment_update, pi_complete: {}, tx: {}", _payment_id.clone(), txid.clone());
    dto = match pi_complete(context.client(), &payment_id, &txid).await {
      Ok(c) => {
        completed = true;
        println!("pi_payment_update, pi_complete return dto: {} {}, completed: {}", _payment_id.clone(), c.amount, c.status.developer_completed.clone());
        Some(c)
      }
      Err(_e) => {
        let err_str = format!(
          "Pi Server: complete payment error {}, paymentid {} error: {}",
          _pi_user_alias.clone(),
          _payment_id.clone(),
          _e.to_string()
        );
        println!("pi_payment_update, {}", err_str.clone());
        return Err(LemmyError::from_message(&err_str));
      },
    };
  }

  if !exist || !approved {
  } else {
  }

  let mut _payment_dto = PiPaymentDto {
    ..PiPaymentDto::default()
  };

  if dto.is_some() {
    if completed && person.is_some() && !verified {
      match Person::update_kyced(context.pool(), person.unwrap().id).await {
        Ok(p) =>{
          println!("pi_payment_update, verify user {}", _pi_user_alias.clone());
        }
        Err(e) => {
          println!("pi_payment_update, verify user err {}", e.to_string());          
        }
      }
    }
    _payment_dto = dto.unwrap();
  }

  let create_at = match chrono::NaiveDateTime::parse_from_str(&_payment_dto.created_at, "%Y-%m-%dT%H:%M:%S%.f%Z")
  {
      Ok(dt) => Some(dt),
      Err(_e) => {
        None
      }
  };

  completed = _payment_dto.status.developer_completed.clone();
  
  let object_id = approve.object_id.clone();
  if !exist {
    //println!("pi_payment_update, create local clone: {} - {} {} ", _pi_user_alias.clone(), _payment_id.clone(), _payment_dto.memo.clone());
    let mut payment_form = PiPaymentInsertForm::builder()
      .domain(approve.domain.clone())
      //.instance_id(None)
      //.obj_cat(Some(comment))
      .obj_id(object_id.clone())
      //.other_id(None)
      //.notes(None)

      .ref_id(None)
      .comment(None)
      .person_id( person_id.clone())
      .testnet( context.settings().pinetwork.pi_testnet)
      
      .finished( false)
      .updated( None)
      .pi_uid( _pi_uid)
      .pi_username( _pi_user_alias.clone())      
      
      .identifier( payment_id.clone())
      .user_uid( _payment_dto.user_uid)
      .amount( _payment_dto.amount)
      .memo( _payment_dto.memo.clone())
      .from_address( _payment_dto.from_address)
      .to_address( _payment_dto.to_address)
      .direction( _payment_dto.direction)
      .network( _payment_dto.network)
      .created_at( create_at)
      .approved( _payment_dto.status.developer_approved)
      .verified( _payment_dto.status.transaction_verified)
      .completed( _payment_dto.status.developer_completed)
      .cancelled( _payment_dto.status.cancelled)
      .user_cancelled( _payment_dto.status.user_cancelled)
      .tx_link("".to_string())
      .tx_id( "".to_string())
      .tx_verified( false)
      .metadata( None) //_payment_dto.metadata,
      .extras( None)
      .build();

    match _payment_dto.transaction {
      Some(tx) => {
        payment_form.tx_link = tx._link;
        payment_form.tx_verified = tx.verified;
        payment_form.tx_id = tx.txid;
        payment_form.finished = true;
      }
      None => {}
    }
    let payment = match PiPayment::create(context.pool(), &payment_form).await
    {
      Ok(payment) => {
        pid = payment.id;
        println!("pi_payment_update, create payment success: {}", _payment_id.clone());
        Some(payment)
      }
      Err(_e) => {
        let err_str = _e.to_string();
        println!("pi_payment_update, create payment error: {} {} ", _payment_id.clone(), err_str.clone());
        return Err(LemmyError::from_message(&err_str));
      }
    };
    pmt = payment.unwrap();
  } else {
    let mut payment_form = PiPaymentUpdateForm::builder()
        .approved(approved)
        .completed(completed)
        .cancelled(cancelled)
        .user_cancelled(usercancelled)
        .build();
    payment_form.updated = Some(naive_now());
    if object_id.is_none() {
      payment_form.metadata = _payment_dto.metadata;
    }
    match _payment_dto.transaction {
      Some(tx) => {
        payment_form.tx_link = tx._link;
        payment_form.tx_verified = tx.verified;
        payment_form.tx_id = tx.txid;
        payment_form.finished = true;
      }
      None => {}
    }
    payment_form.finished = true;
    //println!("Update blockchain memo:{} id:{} link:{}", payment_form.memo.clone(), comment2.clone(), payment_form.tx_link.clone());
    // TODO: UUID check
    if completed {      
      if _payment_dto.memo.clone() == "page" {
        let link = Some(payment_form.tx_link.clone());
        let link2 = payment_form.tx_link.clone();
        let uuid = object_id.clone();
        match uuid {
          Some(u) => {
            let post_id = PostId(u);
            let updated_post = match Post::update_tx(context.pool(), post_id, &link.unwrap_or("".to_string())) .await
            {
              Ok(p) => {
                Some(p)
              }
              Err(_e) => None,
            };
          },
          None => {
            //None
          }
        };
      } else if _payment_dto.memo.clone() == "note" {
        let link = Some(payment_form.tx_link.clone());
        let link2 = payment_form.tx_link.clone();
        let uuid = object_id.clone();
        match uuid {
          Some(u) => {
            let comment_id = CommentId(u);
            let updated_comment = match Comment::update_tx(context.pool(), comment_id, &link.unwrap_or("".to_string())).await
            {
              Ok(c) => {
                Some(c)
              }
              Err(_e) => None,
            };
          },
          None => {
          }
        };
      }
    }
    //pmt = _payment.unwrap();
    pid = pipayment.unwrap().id;
    let payment = match PiPayment::update(context.pool(), pid, &payment_form).await
    {
      Ok(p) => {
        println!("pi_payment_update, update payment success: {} {}", _payment_id.clone(), p.completed);
        Some(p)
      },
      Err(_e) => {
        let err_str = _e.to_string();
        println!("pi_payment_update, update payment error: {} {} ", _payment_id.clone(), err_str.clone());
        return Err(LemmyError::from_message(&err_str));
      }
    };
    pmt = payment.unwrap();
  }
  return Ok(pmt);
}
