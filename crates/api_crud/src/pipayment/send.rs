use crate::pipayment::client::*;
use crate::PerformCrud;
use actix_web::web::Data;
use lemmy_api_common::utils::{get_local_user_view_from_jwt, is_admin};
use lemmy_api_common::{context::LemmyContext};
use lemmy_api_common::pipayment::*;

use lemmy_db_schema::newtypes::{PersonId, PiUserId};
use lemmy_db_schema::source::person::Person;
use lemmy_db_schema::source::pipayment::{PiPayment, PiPaymentSafe, PiPaymentUpdatePending};
use lemmy_utils::{error::LemmyError, ConnectionId};
use lemmy_db_schema::traits::Crud;
use uuid::Uuid;

#[async_trait::async_trait(?Send)]
impl PerformCrud for SendPayment {
  type Response = SendPaymentResponse;
  async fn perform(
    &self,
    context: &Data<LemmyContext>,
    websocket_id: Option<ConnectionId>,
  ) -> Result<SendPaymentResponse, LemmyError> {
    let data: &SendPayment = self;
    let local_user_view =
      get_local_user_view_from_jwt(&data.auth, context.pool(), context.secret()).await?;

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
        println!("pi_incompleted_server_payments error: {}", _e.to_string());
        return Err(LemmyError::from_message("Server busy!"));
      }
    };

    let payment = match PiPayment::find_pending(context.pool(), data.id).await
    {
      Ok(pay) => {
        println!("Sending payment found : {}, cat: {}, identifier {}", pay.id.clone(), pay.obj_cat.clone().unwrap_or_default(),  pay.identifier.clone().unwrap_or_default());
        pay
      },
      Err(_e) => {
        println!("SendPayment error: {}", _e.to_string());
        return Err(LemmyError::from_message("Payment not found!"));
      }
    };

    let person = Person::read(context.pool(), payment.person_id.clone().unwrap_or_default()).await?;
    // let uuid = Uuid::parse_str(&person.external_id.clone().unwrap());
    // let puid = match uuid {
    //   Ok(u) => Some(PiUserId(u)),
    //   Err(_e) => {
    //     return Err(LemmyError::from_message("User not found!"));
    //   }
    // };
    // if !person.verified {
    //   return Err(LemmyError::from_message("User not verified!"));
    // }

    if payment.step == 0 {
      PiPayment::update_step(context.pool(), payment.id.clone(), 1).await?;
    }
    if payment.identifier.is_none() {
      let args = PiPaymentArgs {
        amount: payment.amount,
        uid: person.external_id.clone().unwrap(),
        memo: Some("withdraw".to_string()),
        metadata: None,
      };
      println!("SendPayment for: {} {}", person.external_id.clone().unwrap(), payment.user_uid.clone().unwrap_or_default());
      let dto = match pi_create(context.client(), &args).await
      {
        Ok(pay) => {
          println!("Sending payment, create : from: {}, to: {}, identifier {}", pay.from_address.clone(), pay.to_address.clone(),  pay.identifier.clone());
          pay
        },
        Err(_e) => {
          println!("SendPayment error: {}", _e.to_string());
          return Err(LemmyError::from_message("CreatePayment error!"));
        }
      };
      let approved = dto.status.developer_approved;
      let completed = dto.status.developer_completed;
      let cancelled = dto.status.cancelled;
      let usercancelled = dto.status.user_cancelled;
      
      let finished = false;
      
      let create_at = match chrono::NaiveDateTime::parse_from_str(&dto.created_at, "%Y-%m-%dT%H:%M:%S%.f%Z")
      {
          Ok(dt) => Some(dt),
          Err(_e) => {
            None
          }
      };

      let form = PiPaymentUpdatePending::builder()
        .identifier(Some(dto.identifier))
        .step(2)
        .from_address(Some(dto.from_address))
        .to_address(Some(dto.to_address))
        .direction(Some(dto.direction))
        .created_at(create_at)
        .network(Some(dto.network))
        .approved(approved)
        .completed(completed)
        .cancelled(cancelled)
        .user_cancelled(usercancelled)
        .build();
        PiPayment::update_pending(context.pool(), payment.id.clone(), &form).await?;
    }
    /// TODO: Check user balances > amount 
    
    Ok(SendPaymentResponse {      
      success: false,
      id: Some(payment.id.clone()),
      payment: None,
    })
  }
}
