use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct PiApprove {
  pub paymentid: String,
  pub username: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct PiResponse {
  pub paymentid: String,
  pub username: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct PiTip {
  pub txid: String,
  pub username: String,
  pub paymentid: String,
  pub auth: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct PiTipResponse {
  pub paymentid: String,
}
