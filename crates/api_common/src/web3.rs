use crate::person::*;
use serde::{Deserialize};
use uuid::Uuid;

#[derive(Clone, Deserialize)]
pub struct ExternalAccount {
  pub provider: Option<String>,
  pub account: String,    // Web3 address / Pi Network username / Google sub ...
  pub token: String,      // Secure token /  Pi token / Google token
  pub signature: Option<String>,  // Web3 signature / Pi payment id   
  pub epoch: i64,
  pub uuid: Option<Uuid>,
  pub extra: Option<String>,  // Pi uid, refresh token / txid
  pub comment: Option<String>,
}

#[derive(Deserialize)]
pub struct Web3Register {
  pub ea: ExternalAccount,
  pub info: Register,
}

#[derive(Deserialize)]
pub struct Web3Login {
  pub account: String,
  pub token: String,
  pub signature: String,
  pub epoch: i64,
  pub info: Option<Login>,
}




