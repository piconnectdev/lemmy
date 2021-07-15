use serde::{Deserialize, Serialize};
use serde_json::Value;
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

#[derive(Deserialize, serde::Serialize, Debug, Default)]
pub struct PiPaymentStatus {
  pub developer_approved: bool,
  pub transaction_verified: bool,
  pub developer_completed: bool,
  pub cancelled: bool,
  pub user_cancelled: bool,
}

#[derive(Deserialize, serde::Serialize, Debug, Default)]
pub struct PiPaymentTransaction {
  pub txid: Option<String>,
  pub verified: bool,
  pub link: String,
}

#[derive(Deserialize, serde::Serialize, Debug, Default)]
pub struct PiPaymentDto {
  pub identifier: String,
  pub user_uid: String,
  /// uuid::Uuid
  pub amount: f64,
  pub memo: String,
  pub to_address: String,
  //pub created_at: chrono::NaiveDateTime,
  pub status: PiPaymentStatus,
  pub transaction: Option<PiPaymentTransaction>,
  pub metadata: Option<Value>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TxRequest {
  pub txid: String,
}
