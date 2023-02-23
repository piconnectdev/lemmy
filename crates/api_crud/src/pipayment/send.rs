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
use stellar_sdk::types::Operation;
use uuid::Uuid;

use stellar_sdk::{CallBuilder, Server, types::{Asset, Transaction}, utils::{Direction, Endpoint}};

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

    let mut identifier;
    let mut from_address;
    let mut to_address;
    let mut network;

    let mut payment = match PiPayment::find_pending(context.pool(), data.id).await
    {
      Ok(pay) => {
        println!("Sending payment found : {}, cat: {}, identifier {}", pay.id.clone(), pay.obj_cat.clone().unwrap_or_default(),  pay.identifier.clone().unwrap_or_default());
        identifier = pay.identifier.clone().unwrap_or_default();
        from_address = pay.identifier.clone().unwrap_or_default();
        to_address = pay.to_address.clone().unwrap_or_default();
        network = pay.network.clone().unwrap_or_default();
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
    let mut dto;
    if payment.identifier.is_none() {
      let args = PiPaymentCreate{
          payment: PiPaymentArgs {
          amount: payment.amount,
          uid: person.external_id.clone().unwrap(),
          memo: Some("withdraw".to_string()),
          metadata: None,
          },
      };
      println!("SendPayment for: {} {}", person.external_id.clone().unwrap(), payment.user_uid.clone().unwrap_or_default());
      dto = match pi_create(context.client(), &args).await
      {
        Ok(pay) => {
          println!("Sending payment, create : from: {}, to: {}, identifier {}", pay.from_address.clone(), pay.to_address.clone(),  pay.identifier.clone());
          identifier = pay.identifier.clone();
          from_address = pay.identifier.clone();
          to_address = pay.to_address.clone();
          network = pay.network.clone();
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
        .identifier(Some(dto.identifier.clone()))
        .step(2)
        .from_address(Some(dto.from_address.clone()))
        .to_address(Some(dto.to_address.clone()))
        .direction(Some(dto.direction))
        .created_at(create_at)
        .network(Some(dto.network.clone()))
        .approved(approved)
        .completed(completed)
        .cancelled(cancelled)
        .user_cancelled(usercancelled)
        .build();
        payment = PiPayment::update_pending(context.pool(), payment.id.clone(), &form).await?;
    }
    
    let txdata = TransactionData{
      amount: payment.amount,
      payment_id: identifier.clone(),
      from_address: from_address,
      to_dddress: to_address,
    };

    let server = get_horizon_client(&network.clone()).await?;
    // let tx = build_a2u_transaction(&server, &txdata).await?;
    // let tx = submit_transaction(&server, &tx).await?;
    // if tx.successful {
    //   let dto = match pi_complete(context.client(), &identifier.clone(), &tx.id).await
    //   {
    //     Ok(p) => {
    //       Some(p)
    //     },
    //     Err(e) => {
    //       None
    //     }
    //   };
    // }

    Ok(SendPaymentResponse {      
      success: false,
      id: Some(payment.id.clone()),
      payment: None,
    })
  }
}

pub struct TransactionData {
  amount: f64,
  payment_id: String,
  from_address: String,
  to_dddress: String,
}


pub async fn get_horizon_client(network: &str) -> Result<Server, LemmyError> {
  //let server = (network == "Pi Network") ? "https://api.mainnet.minepi.com" : "https://api.testnet.minepi.com";
  let server;
  if network == "Pi Network" {
    server = "https://api.mainnet.minepi.com";
  } else {
    server = "https://api.testnet.minepi.com";
  }
  return Ok(Server::new(server.to_owned()));
}

pub async fn build_a2u_transaction(server: &Server, data: &TransactionData) -> Result<Transaction, LemmyError> {
  // let pay_ops = Operation{
  //   re: data.to_dddress,
  //   asset: Asset::native(),
  // };
  // let paymentOperation = Server.Operation.payment({
  //   destination: transactionData.toAddress,
  //   asset: StellarSdk.Asset.native(),
  //   amount: transactionData.amount.toString(),
  // });
  // const paymentOperation = StellarSdk.Operation.payment({
  //   destination: transactionData.toAddress,
  //   asset: StellarSdk.Asset.native(),
  //   amount: transactionData.amount.toString(),
  // });

  // const transaction = new StellarSdk.TransactionBuilder(myAccount, {
  //   fee: baseFee.toString(),
  //   networkPassphrase: this.NETWORK_PASSPHRASE,
  //   timebounds: await piHorizon.fetchTimebounds(180),
  // })
  //   .addOperation(paymentOperation)
  //   .addMemo(StellarSdk.Memo.text(transactionData.paymentIdentifier))
  //   .build();

  // transaction.sign(this.myKeypair);
  // return transaction;
  return Err(LemmyError::from_message("CreatePayment error!"));
}

pub async fn submit_transaction(server: &Server, tx: &Transaction) -> Result<Transaction, LemmyError> {
  //let txresult = server.submitTransaction();
  return Err(LemmyError::from_message("CreatePayment error!"));
}
