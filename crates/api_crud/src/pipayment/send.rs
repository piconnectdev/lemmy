use crate::pipayment::client::*;
use crate::PerformCrud;
use actix_web::web::Data;
use lemmy_api_common::utils::{local_user_view_from_jwt, is_admin};
use lemmy_api_common::{context::LemmyContext};
use lemmy_api_common::pipayment::*;

use lemmy_db_schema::newtypes::{PersonId, PiUserId, PiPaymentId};
use lemmy_db_schema::source::person::Person;
use lemmy_db_schema::source::pipayment::{PiPayment, PiPaymentUpdatePending, PiPaymentUpdateForm};
use lemmy_db_schema::utils::naive_now;
use lemmy_utils::{error::LemmyError, };
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
  ) -> Result<SendPaymentResponse, LemmyError> {
    let data: &SendPayment = self;
    let local_user_view =
      local_user_view_from_jwt(&data.auth, context).await?;

    is_admin(&local_user_view)?;
    return Err(LemmyError::from_message("Server send temporary disabled!"));

    let _pays = match pi_incompleted_server_payments(context.client()).await
    {
      Ok(pays) => {
        println!("incompleted_server_payments found {}", pays.len());
        if !pays.is_empty() {
          let mut pay_iter = pays.iter();
          for pay in pay_iter {
            let mut step = 0;
            let mut id: Option<PiPaymentId> = None;
            let mut exist = false;
            let mut meta_exist = false;
            
            if pay.metadata.is_some() {
              meta_exist = true;
              let meta = pay.metadata.clone().unwrap();
              if meta.cat.unwrap_or_default() == "withdraw" && meta.id.is_some() {
                exist = true;
                id = Some(PiPaymentId(meta.id.unwrap()));
              }              
            }

            let mut payment = match PiPayment::find_by_pipayment_id(context.pool(), &pay.identifier.clone()).await
            {
              Ok(p) => {
                println!("incompleted_server_payments payment by identifier: {}", p.identifier.clone().unwrap_or_default());
                step = p.step;
                id = Some(p.id.clone());
                exist = true;
                Some(p)
              },
              Err(e) => {
                None
              },
            };
            if payment.is_none() {              
              if id.is_some() {                
                payment = match PiPayment::read(context.pool(), id.clone().unwrap_or_default()).await 
                {
                    Ok(p) => {
                      println!("incompleted_server_payments payment by id: {}", id.clone().unwrap_or_default());
                      step = p.step;
                      exist = true;
                      Some(p)
                    },
                    Err(e) => {
                      None
                    }
                };
              }
            }
            if payment.is_none() {
              // Unknown payment, cancel
              println!("cancel incompleted payment from server: {} - {} approved:{} completed:{} cancelled:{} completed:{} user_cancelled:{}",
                      pay.identifier.clone(), pay.user_uid, 
                      pay.status.developer_approved, pay.status.developer_completed, pay.status.cancelled, pay.status.transaction_verified, pay.status.user_cancelled);
              // if pay.status.developer_completed == false && pay.status.cancelled == false {
              //   match pi_cancel(context.client(), &pay.identifier.clone()).await
              //   {
              //     Ok(p) => {
              //       println!("Cancel payment {}", &pay.identifier.clone());
              //     },
              //     Err(e) =>{
              //       println!("Cancel payment {}", &pay.identifier.clone());
              //     }
              //   }
              // }
              continue;
            } 
            println!("process incompleted payment from server: {} - {} approved:{} completed:{} cancelled:{} completed:{} user_cancelled:{}",
              pay.identifier.clone(), pay.user_uid.clone(), 
              pay.status.developer_approved, pay.status.developer_completed, pay.status.cancelled, pay.status.transaction_verified, pay.status.user_cancelled);
            
            let payment = payment.unwrap();
            let id = payment.id.clone();
            if pay.direction != "app_to_user" {
              continue;
            }

            let mut txverified = false;
            let mut txlink = None;
            let mut txid = None;
            if pay.transaction.is_some() { 
              let txo = pay.transaction.clone().unwrap();   
              txverified = txo.verified;
              txlink = Some(txo._link);
              txid = Some(txo.txid);
            }
            let create_at = match chrono::NaiveDateTime::parse_from_str(&pay.created_at, "%Y-%m-%dT%H:%M:%S%.f%Z")
            {
                Ok(dt) => Some(dt),
                Err(_e) => {
                  None
                }
            };
            let serialized_meta = serde_json::to_string(&pay.metadata.clone()).unwrap();
            let metadata = serde_json::to_value(serialized_meta).unwrap();
            // let mut object: PiPaymentMeta = serde_json::from_str(input).unwrap();
            let mut must_update = false;
            if payment.identifier.is_none() {
              if step == 0 || step == 1 {
                step = 2;
              }
              must_update = true;
            }
            if must_update {
              println!("update incompleted payment from server: {} - {} ", pay.identifier.clone(), pay.user_uid.clone(), );
              let form = PiPaymentUpdateForm::builder()
                .step(step)
                //.finished(false)
                .identifier(Some(pay.identifier.clone()))
                .user_uid(Some(pay.user_uid.clone()))
                .amount(pay.amount)
                .memo(Some(pay.memo.clone()))
                .direction(Some(pay.direction.clone()))
                .created_at(create_at)
                .network(Some(pay.network.clone()))
                .extras(None)
                .from_address(Some(pay.from_address.clone()))
                .to_address(Some(pay.to_address.clone()))
                .approved(pay.status.developer_approved)
                .completed(pay.status.developer_completed)
                .cancelled(pay.status.cancelled)
                .user_cancelled(pay.status.user_cancelled)
                .verified(pay.status.transaction_verified)
                .tx_verified(txverified)
                .tx_link(txlink)
                .tx_id(txid)
                .updated(Some(naive_now()))
                .metadata(Some(metadata))
                .build();
              let payment = match PiPayment::update(context.pool(), id, &form).await
              {
                Ok(p) => {
                  Some(p)
                },
                Err(e) =>{
                  None
                }
              };
            }
            if step == 2 {
              println!("waiting make transactions for incompleted payment: {} - {} ", pay.identifier.clone(), pay.user_uid.clone(), );
            }
          } // End for loop
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
    let mut pay;
    if payment.identifier.is_none() && (payment.step == 0 || payment.step == 1 ) {
      let meta: PiPaymentMeta = PiPaymentMeta{
        id: Some(payment.id.clone().0),
        cat: Some("withdraw".to_string()),
        data: None
      };
      let serialized_meta = serde_json::to_string(&meta).unwrap();
      let metadata = serde_json::to_value(serialized_meta).unwrap();
      let args = PiPaymentCreate{
          payment: PiPaymentArgs {
            amount: payment.amount,
            uid: person.external_id.clone().unwrap(),
            memo: "withdraw".to_string(),
            metadata: meta.clone(),
          },
      };
      let serialized_meta = serde_json::to_string(&args.clone()).unwrap();
      println!("SendPayment for: {} {} data: {}", person.external_id.clone().unwrap(), payment.user_uid.clone().unwrap_or_default(), serialized_meta);
      pay = match pi_create(context.client(), &args).await
      {
        Ok(dto) => {
          println!("Sending payment, create : from: {}, to: {}, identifier {}", dto.from_address.clone(), dto.to_address.clone(),  dto.identifier.clone());          
          dto
        },
        Err(_e) => {
          return Err(LemmyError::from_message("CreatePayment error!"));
        }
      };
     
      
      let create_at = match chrono::NaiveDateTime::parse_from_str(&pay.created_at, "%Y-%m-%dT%H:%M:%S%.f%Z")
      {
          Ok(dt) => Some(dt),
          Err(_e) => {
            None
          }
      };
      let step = 2;
      let form = PiPaymentUpdateForm::builder()
        .step(step)
        //.finished(false)

        .identifier(Some(pay.identifier.clone()))
        .user_uid(Some(pay.user_uid.clone()))
        .amount(pay.amount)
        .memo(Some(pay.memo.clone()))
        .direction(Some(pay.direction.clone()))
        .created_at(create_at)
        .network(Some(pay.network.clone()))
        .extras(None)
        .from_address(Some(pay.from_address.clone()))
        .to_address(Some(pay.to_address.clone()))
        .approved(pay.status.developer_approved)
        .completed(pay.status.developer_completed)
        .cancelled(pay.status.cancelled)
        .user_cancelled(pay.status.user_cancelled)
        .verified(pay.status.transaction_verified)
        .tx_verified(false)
        .tx_link(None)
        .tx_id(None)
        .updated(Some(naive_now()))
        .metadata(Some(metadata))
        .build();
      let payment = match PiPayment::update(context.pool(), payment.id.clone(), &form).await
      {
        Ok(p) => {
          Some(p)
        },
        Err(e) =>{
          None
        }
      };
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


    // let serialized_user = serde_json::to_string(&user).unwrap();
    // let v: PiPaymentMeta = serde_json::to_value(u).unwrap();
    // let mut object: PiPaymentMeta = serde_json::from_str(input).unwrap();

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
