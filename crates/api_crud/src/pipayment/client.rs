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

  let res: IncompleteServerPayments = response
    .json::<IncompleteServerPayments>()
    .await
    .map_err(|e| LemmyError::from_error_message(e, ""))?;
  Ok(res.incomplete_server_payments)
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
  // let content = response.text().await?;
  // println!("pi_create: {}", content.clone());
  // return Err(LemmyError::from_message("Can not create A2U payment"));
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



/*
pub async fn pi_dopayment(
  context: &Data<LemmyContext>,
) -> Result<PiPayment, LemmyError> {

  let local_user_view =
  get_local_user_view_from_jwt(&data.auth, context.pool(), context.secret()).await?;
let local_site = LocalSite::read(context.pool()).await?;

// Make sure user is an admin
is_admin(&local_user_view)?;

    let _pays = match pi_incompleted_server_payments(context.client()).await
    {
      Ok(pays) => {
        if !pays.is_empty() {
          let mut pay_iter = pays.iter();
          for pay in pay_iter {
            if pay.transaction.is_some() {
              println!("Got completed: {}", pay.identifier);
              // match pi_complete(context.client(), pay.identifier).await
              // {
              // }
            } else {
              println!("Got completed: {}", pay.status.developer_approved);
              // match pi_cancel(context.client(), pay.identifier).await
              // {

              // };
            }
          }
        }
      },
      Err(_e) => {
        return Err(LemmyError::from_message("Server busy!"));
      }
    };
    /// TODO: Check user balances > amount 
    let args = PiPaymentArgs {
      amount: 0.01,
      //amount: data.amount,
      //pub memo: String,
      //pub metadata: Option<Value>,
      uid: person.external_id.clone().unwrap(),
      memo: data.comment.clone(),
      metadata: None,
    };

    // let payment = match pi_create(context.client(), &args).await
    // {
    //   Ok(c) => {
    //     _payment_id = c.identifier.clone();
    //     Some(c)
    //   }
    //   Err(_e) => {
    //     return Err(LemmyError::from_message("Not approved payment"));
    //   },
    // };
    // TODO: Submit transaction
    // TODO: Completed transaction
    //println!("PiWithdrawResponse: {} {}", person_id.clone(), paymentid.clone());

}

 */