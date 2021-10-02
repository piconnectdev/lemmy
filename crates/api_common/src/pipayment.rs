use lemmy_db_schema::{PaymentId, PiUserId,};
use crate::person::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct PiPaymentFound {
  pub paymentid: String,
  pub pi_username: String,
  pub pi_uid: Option<PiUserId>,
  pub auth: Option<String>,
  pub dto: Option<PiPaymentDto>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct PiPaymentFoundResponse {
  pub id: PaymentId,
  pub paymentid: String,
}

#[derive(Deserialize)]
pub struct PiAgreeRegister {
  pub paymentid: String,
  pub pi_username: String,
  pub pi_uid: Option<PiUserId>,
  pub comment: Option<String>,
  pub info: Register,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct PiAgreeResponse {
  pub success: bool,
  pub id: Option<PaymentId>,
  pub paymentid: String,
  pub extra: Option<String>,
}

#[derive(Deserialize)]
pub struct PiRegister {
  pub paymentid: String,
  pub pi_username: String,
  pub pi_uid: Option<PiUserId>,
  pub comment: Option<String>,
  pub txid: String,
  pub info: Register,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct PiRegisterResponse {
  pub success: bool,
  pub jwt: String,
  pub extra: Option<String>,
}

#[derive(Deserialize)]
pub struct PiLogin {
  pub pi_username: String,
  pub pi_uid: PiUserId,
  pub pi_token: String,
  pub info: Option<Login>,
}


#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct PiApprove {
  pub paymentid: String,
  pub pi_username: String,
  pub pi_uid: Option<PiUserId>,
  pub person_id: Option<Uuid>,
  pub comment: Option<String>,
  pub auth: Option<String>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct PiApproveResponse {
  pub id: PaymentId,
  pub paymentid: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct PiTip {
  pub paymentid: String,
  pub pi_username: String,
  pub pi_uid: Option<PiUserId>,
  pub person_id: Option<Uuid>,
  pub comment: Option<String>,
  pub txid: String,
  pub auth: Option<String>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct PiTipResponse {
  pub id: PaymentId,
  pub paymentid: String,
}

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct PiPaymentStatus {
  pub developer_approved: bool,
  pub transaction_verified: bool,
  pub developer_completed: bool,
  pub cancelled: bool,
  pub user_cancelled: bool,
}

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct PiPaymentTransaction {
  pub txid: String,
  pub verified: bool,
  pub _link: String,
}

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct PiPaymentDto {
  pub identifier: String,
  pub user_uid: String,
  pub amount: f64,
  pub memo: String,
  pub to_address: String,
  pub created_at: String,
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

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct PiUserDto {
  pub uid: PiUserId,
  pub username: String,
}
