use clokwerk::{Scheduler, TimeUnits};
// Import week days and WeekDay
use diesel::{sql_query, PgConnection, RunQueryDsl};
use diesel::{Connection, ExpressionMethods, QueryDsl};
use lemmy_db_schema::{
  source::instance::{Instance, InstanceForm},
  utils::naive_now,
};
use lemmy_routes::nodeinfo::NodeInfo;
use lemmy_utils::{error::LemmyError, REQWEST_TIMEOUT};
use reqwest::blocking::Client;
use std::{thread, time::Duration};
use tracing::info;

use crate::SETTINGS;
use crate::build_db_pool;
use lemmy_utils::request::retry;
use lemmy_db_schema::source::pipayment::*;
use lemmy_db_schema::utils::DbPool;
use lemmy_db_schema::utils::get_conn;
use lemmy_api_crud::pipayment::client::*;
use lemmy_api_common::pipayment::PiPaymentDto;
use lemmy_api_common::pipayment::PiPaymentTransaction;
use lemmy_api_common::pipayment::IncompleteServerPayments;
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use reqwest_retry::{RetryTransientMiddleware, policies::ExponentialBackoff};
use futures::{executor};

pub async fn pi_incompleted_payments_async () -> Result<Vec<PiPaymentDto>, LemmyError> {
    let retry_policy = ExponentialBackoff::builder().build_with_max_retries(3);
    let client = ClientBuilder::new(reqwest::Client::new())
        //.with(TracingMiddleware)
        .with(RetryTransientMiddleware::new_with_policy(retry_policy))
        .build();
    let pays = pi_incompleted_server_payments(&client).await?;
    return Ok(pays)
  }
  
  pub fn pi_incompleted_payments(
    user_agent: &str
  ) -> Result<Vec<PiPaymentDto>, LemmyError> {
    let client = Client::builder()
    .user_agent(user_agent)
    .timeout(REQWEST_TIMEOUT)
    .build()
    .expect("couldnt build reqwest client");
  
    let settings = SETTINGS.to_owned();
    let fetch_url = format!("{}/payments/incomplete_server_payments", settings.pi_api_host());
  
    let response = 
      client
        .get(&fetch_url)
        .timeout(REQWEST_TIMEOUT)
        .header("Authorization", format!("Key {}", settings.pi_key()))
        .header("Content-Type", format!("application/json"))
        .send()
        .ok()
        .and_then(|t| t.json::<IncompleteServerPayments>().ok())
        ;
    if response.is_none() {
      return Err(LemmyError::from_message(""))
    }
    let pays = response.unwrap().incomplete_server_payments;
    if pays.is_empty() {
      info!("Pipayment incompleted_payments is empty");
      return Ok(pays);
    }
    let mut pay_iter = pays.iter();
    for pay in pay_iter {
      let str = serde_json::to_string(&pay).unwrap();
      info!("Process pi_incompleted_payments: {} {} ",  pay.identifier, str);
      if pay.direction != "app_to_user" {
        //pi_complete(pay.identifier, );
      }
      if pay.status.transaction_verified == false{
  
      }
      if pay.transaction.is_some() {
        info!("Got completed with link: {} {}", pay.identifier, pay.user_uid);
        // match pi_complete(context.client(), pay.identifier).await
        // {
        // }
      } else {
        info!("Got completed: {}", pay.status.developer_approved);
        // match pi_cancel(context.client(), pay.identifier).await
        // {
  
        // };
      }
    }
    Ok(pays)  
  }
  
  fn pipayment_update_dto(
    pays: &Vec<PiPaymentDto>,
  ) -> Result<i32, LemmyError> {
    if pays.is_empty() {
      return Ok(0);
    }
    let mut pay_iter = pays.iter();
    for pay in pay_iter {  
      let str = serde_json::to_string(&pay).unwrap();
      info!("Process pi_incompleted_payments: {} {} {} ",  pay.identifier, pay.direction, str);
      if pay.direction != "app_to_user" {
        //pi_complete(pay.identifier, );
        return Ok(0);
      }
      if pay.status.cancelled == true {      
        return Ok(0);;
      }
      if pay.status.developer_completed == true {      
        return Ok(0);;
      }
      //let conn = &mut get_conn(pool).await?;
      // let payment = match PiPayment::find_by_pipayment_id(pool, &pay.identifier.clone()).await
      // {
      //   Ok(p) => {
      //     Some(p)
      //   },
      //   Err(e)=>{
      //     None
      //   }
      // };
      if pay.status.transaction_verified == false {
        
        return Ok(0);;
      } else {
        if pay.transaction.is_some() {
          info!("Got completed with link: {} {}", pay.identifier, pay.user_uid);
          // match pi_complete(context.client(), pay.identifier).await
          // {
          // }
        } else {
          info!("Got completed: {}", pay.status.developer_approved);
          // match pi_cancel(context.client(), pay.identifier).await
          // {
  
          // };
        }
      }
    }  
    return Ok(1);
  }
  pub fn pipayment_task(db_url: &str, user_agent: &str) {
    info!("Pipayment start ...");
    // let settings = SETTINGS.to_owned();
    // // Run the migrations from code
    // let pool = match build_db_pool(&settings).await
    // {
    //   Ok(p) => p,
    //   Err(_e) => {
    //     info!("pi_incompleted_server_payments error: {}", _e.to_string());
    //     return;
    //   }
    // };
    let conn = &mut PgConnection::establish(&db_url.clone())
      .unwrap_or_else(|_| panic!("Error connecting to {db_url}"));
  
    match pi_incompleted_payments(&user_agent)
    {
      Ok(pays) => {
        pipayment_update_dto(&pays);
      },
      Err(_e) => {
        info!("pi_incompleted_server_payments error: {}", _e.to_string());        
      }    
    };
    info!("Done.");
  }
  