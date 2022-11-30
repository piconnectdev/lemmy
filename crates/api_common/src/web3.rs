use crate::person::*;
use serde::{Deserialize};
use lemmy_db_schema::newtypes::{PiUserId};

#[derive(Deserialize)]
pub struct Web3Register {
  pub ea: ExternalAccount,
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

#[derive(Clone, Deserialize)]
pub struct ExternalAccount {
  pub provider: Option<String>,
  pub account: String,    // Web3 address / Pi Network username / Google sub ...
  pub token: String,      // Secure token /  Pi token / Google token
  pub signature: String,  // Web3 signature / Pi payment id   
  pub extra: Option<String>,  // Pi uid, refresh token / txid
  pub puid: Option<PiUserId>,
  pub cli_time: i64,
  pub comment: Option<String>,
}


