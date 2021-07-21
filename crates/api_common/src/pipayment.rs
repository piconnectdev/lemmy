use lemmy_db_schema::{PaymentId, PiPaymentId};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct PiPaymentFound {
  pub paymentid: PiPaymentId,
  pub username: String,
  pub auth: Option<String>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct PiPaymentFoundResponse {
  pub paymentid: PiPaymentId,
  pub id: PaymentId,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct PiAgreeRegister {
  pub paymentid: PiPaymentId,
  pub pi_username: Option<String>,
  pub username: String,
  pub password: String,
  pub password_verify: String,
  pub show_nsfw: bool,
  pub email: Option<String>,
  pub captcha_uuid: Option<String>,
  pub captcha_answer: Option<String>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct PiAgreeResponse {
  pub id: PiPaymentId,
  pub username: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct PiRegisterResponse {
  pub jwt: String,
}


#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct PiApprove {
  pub paymentid: PiPaymentId,
  pub username: String,
  pub auth: Option<String>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct PiApproveResponse {
  pub paymentid: PiPaymentId,
  pub username: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct PiTip {
  pub txid: String,
  pub username: String,
  pub paymentid: PiPaymentId,
  pub auth: Option<String>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct PiTipResponse {
  pub paymentid: String,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct PiPaymentStatus {
  pub developer_approved: bool,
  pub transaction_verified: bool,
  pub developer_completed: bool,
  pub cancelled: bool,
  pub user_cancelled: bool,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct PiPaymentTransaction {
  pub txid: Option<String>,
  pub verified: bool,
  pub link: String,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct PiPaymentDto {
  pub identifier: String,
  pub user_uid: String,
  /// uuid::Uuid
  pub amount: f64,
  pub memo: String,
  pub to_address: String,
  pub created_at: Option<chrono::NaiveDateTime>,
  pub status: PiPaymentStatus,
  pub transaction: Option<PiPaymentTransaction>,
  pub metadata: Option<Value>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TxRequest {
  pub txid: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GetPayment {
  pub id: PaymentId,
  pub auth: String,
}

#[derive(Serialize, Debug, Default)]
pub struct GetPaymentResponse {
  pub pid: String,
  //pub dto: PiPaymentDto,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GetPayments {
  pub sort: Option<String>,
  pub page: Option<i64>,
  pub limit: Option<i64>,
  pub auth: String,
}
