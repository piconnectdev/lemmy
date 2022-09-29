use crate::person::*;
use serde::{Deserialize};

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
