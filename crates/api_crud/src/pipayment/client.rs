use actix_web::web::Data;
use lemmy_api_common::pipayment::*;
use lemmy_db_schema::{
  newtypes::{CommentId, *},
  source::{comment::*, pipayment::*, post::*, person::*},
  traits::{Crud, Signable } ,
  utils::naive_now,
};
use lemmy_utils::{error::LemmyError, request::retry, settings::SETTINGS, REQWEST_TIMEOUT};
use lemmy_api_common::{context::LemmyContext};
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

pub async fn pi_me(client: &ClientWithMiddleware, key: &str) -> Result<PiUserDto, LemmyError> {
  let settings = SETTINGS.to_owned();
  let fetch_url = format!("{}/me", settings.pi_api_host());

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
  Ok(res)
}

pub async fn pi_payment_update(
  context: &Data<LemmyContext>,
  approve: &PiApprove,
  tx: Option<String>,
) -> Result<PiPayment, LemmyError> {
  let settings = SETTINGS.to_owned();

  let pi_username = approve.pi_username.clone();
  let pi_uid = approve.pi_uid.clone();
  let payment_id = approve.paymentid.clone();  
  //let comment = approve.comment.clone();
  let comment = approve.comment.clone().unwrap_or("".to_string());
  let mut person_id: Option<PersonId> = None;

  let person = match Person::find_by_extra_name(context.pool(), &pi_username.clone()).await
  {
    Ok(c) => {
      person_id = Some(c.id.clone());
      Some(c)
    },
    Err(_e) => None
  };

  // let local_user_view =
  //   get_local_user_view_from_jwt(&data.auth, context.pool(), context.secret()).await?;
  // let local_site = LocalSite::read(context.pool()).await?;

  // Hide PiUserName
  // let mut sha256 = Sha256::new();
  // sha256.update(pi_username.to_owned());
  // let _pi_user_alias: String = format!("{:X}", sha256.finalize());
  let _pi_user_alias = pi_username;

  let mut _payment_id = format!("{}", payment_id);
  let _pi_uid = pi_uid;

  let mut approved = false;
  let mut completed = false;
  let mut exist = false;
  //let mut fetch_pi_server = true;
  let mut pid;
  let mut pmt;
  let mut _payment = match PiPayment::find_by_pipayment_id(context.pool(), &_payment_id.clone().to_owned()).await
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
      dto = match pi_approve(context.client(), &payment_id).await {
        Ok(c) => Some(c),
        Err(_e) => None,
      };
    } else if !completed {
      dto = match pi_complete(context.client(), &payment_id, &tx.unwrap()).await {
        Ok(c) => {
          completed = true;
          Some(c)
        }
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

  let mut _payment_dto = PiPaymentDto {
    ..PiPaymentDto::default()
  };

  if dto.is_some() {
    _payment_dto = dto.unwrap();
  }

  let refid = person_id;
  let create_at = match chrono::NaiveDateTime::parse_from_str(&_payment_dto.created_at, "%Y-%m-%dT%H:%M:%S%.f%Z")
  {
      Ok(dt) => Some(dt),
      Err(_e) => {
        // let err_type = format!(
        //   "Pi Server: get payment datetime error: user {}, paymentid {} {} {}",
        //   &pi_username,
        //   &_payment_dto.identifier.clone(),
        //   _payment_dto.created_at,
        //   _e.to_string()
        // );
        //return Err(LemmyError::from_message(&err_type));
        None
      }
  };

  completed = _payment_dto.status.developer_completed.clone();
  
  let refid =  match person_id {
    Some(x) => Some(x.0),
    None => None,
  };

  if !exist {
    let mut payment_form = PiPaymentInsertForm::builder()
    .person_id( person_id.clone())
    .ref_id(refid)
    .testnet( settings.pinetwork.pi_testnet)
    .finished( false)
    .updated( None)
    .pi_uid( _pi_uid)
    .pi_username( _pi_user_alias.clone())
    .comment( Some(comment))

    .identifier( payment_id.clone())
    .user_uid( _payment_dto.user_uid)
    .amount( _payment_dto.amount)
    .memo( _payment_dto.memo)
    .to_address( _payment_dto.to_address)
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
    _payment = match PiPayment::create(context.pool(), &payment_form).await
    {
      Ok(payment) => {
        pid = payment.id;
        Some(payment)
      }
      Err(_e) => {
        let err_type = _e.to_string();
        return Err(LemmyError::from_message(&err_type));
      }
    };
    pmt = _payment.unwrap();
  } else {
    let mut payment_form = PiPaymentUpdateForm::builder()
        .build();
    payment_form.updated = Some(naive_now());
    match _payment_dto.transaction {
      Some(tx) => {
        payment_form.tx_link = tx._link;
        payment_form.tx_verified = tx.verified;
        payment_form.tx_id = tx.txid;
        payment_form.finished = true;
      }
      None => {}
    }
    
    //println!("Update blockchain memo:{} id:{} link:{}", payment_form.memo.clone(), comment2.clone(), payment_form.tx_link.clone());
    // TODO: UUID check
    if completed {
      payment_form.finished = true;
      if payment_form.memo == "post" {
        let link = Some(payment_form.tx_link.clone());
        let link2 = payment_form.tx_link.clone();
        let uuid = Uuid::parse_str(&comment.clone());
        match uuid {
          Ok(u) => {
            let post_id = PostId(u);
            let updated_post = match Post::update_tx(context.pool(), post_id, &link.unwrap_or("".to_string())) .await
            {
              Ok(p) => {
                println!(
                  "Post: Success update blockchain link, id:{} link:{}",
                  comment.clone(),
                  link2.clone()
                );
                Some(p)
              }
              Err(_e) => None,
            };
          }
          Err(e) => {
            //None
          }
        };
      } else if payment_form.memo == "comment" {
        let link = Some(payment_form.tx_link.clone());
        let link2 = payment_form.tx_link.clone();
        // TODO: UUID check
        let uuid = Uuid::parse_str(&comment.clone());
        match uuid {
          Ok(u) => {
            let comment_id = CommentId(u);
            let updated_comment = match Comment::update_tx(context.pool(), comment_id, &link.unwrap_or("".to_string())).await
            {
              Ok(c) => {
                println!(
                  "Comment: Success update blockchain link, id:{} link:{}",
                  c.id,
                  link2.clone()
                );
                Some(c)
              }
              Err(_e) => None,
            };
          }
          Err(e) => {}
        };
      }
    }
    pmt = _payment.unwrap();
    pid = pmt.id;
    let updated_payment = match PiPayment::update(context.pool(), pid, &payment_form).await
    {
      Ok(payment) => Some(payment),
      Err(_e) => {
        let err_type = _e.to_string();
        //return LemmyError::from_error_message(e, &err_type)?;
        //    .map_err(|e| LemmyError::from_error_message(e, &e.to_string().clone()))?;
        return Err(LemmyError::from_message(&err_type));
      }
    };
  }
  return Ok(pmt);
}
