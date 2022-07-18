use crate::person::*;
use lemmy_db_schema::newtypes::{PaymentId, PiUserId};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct Web3Register {
  pub address: String,
  pub signature: String,
  pub token: String,
  pub cli_time: i64,
  pub info: Register,
}

#[derive(Deserialize)]
pub struct Web3Login {
  pub address: String,
  pub signature: String,
  pub token: String,
  pub cli_time: i64,
  pub info: Option<Login>,
}
