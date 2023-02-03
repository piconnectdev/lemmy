use crate::{person::*, web3::ExternalAccount, sensitive::Sensitive};
use lemmy_db_schema::{newtypes::{PiPaymentId, PiUserId}, source::pipayment::PiPaymentSafe};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct PiUserDto {
  pub uid: PiUserId,
  pub username: String,
}

#[derive(Clone, Deserialize)]
pub struct PiPaymentFound {
  pub domain: Option<String>,
  pub pi_username: String,
  pub pi_uid: Option<PiUserId>,
  pub pi_token: Option<String>,
  pub paymentid: String,
  pub auth: Option<String>,
  pub dto: Option<PiPaymentDto>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct PiPaymentFoundResponse {
  pub id: PiPaymentId,
  pub paymentid: String,
}

#[derive(Clone, Deserialize)]
pub struct PiAgreeRegister {
  pub domain: Option<String>,
  pub ea: ExternalAccount,
  pub info: Register,
  pub paymentid: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct PiAgreeResponse {
  pub success: bool,
  pub id: Option<PiPaymentId>,
  pub paymentid: String,
  pub extra: Option<String>,
}

#[derive(Clone, Deserialize)]
pub struct PiRegisterWithFee {
  pub domain: Option<String>,  
  pub ea: ExternalAccount,
  pub paymentid: String,
  pub txid: String,
  pub info: Register,
}

#[derive(Clone, Deserialize)]
pub struct PiRegister {
  pub domain: Option<String>,  
  pub ea: ExternalAccount,
  pub info: Register,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct PiRegisterResponse {
  pub success: bool,
  pub login: LoginResponse,
  pub extra: Option<String>,
}

#[derive(Clone, Deserialize)]
pub struct PiLogin {
  pub domain: Option<String>,  
  pub ea: ExternalAccount,
  pub info: Option<Login>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct PiApprove {
  pub domain: Option<String>,  
  pub pi_username: String,
  pub pi_uid: Option<PiUserId>,
  pub pi_token: Option<String>,
  pub object_id: Option<Uuid>,
  pub paymentid: String,
  pub comment: Option<String>,
  pub auth: Option<String>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct PiApproveResponse {
  pub success: bool,
  pub id: PiPaymentId,
  pub paymentid: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct PiPaymentComplete {
  pub domain: Option<String>,  
  pub pi_username: String,
  pub pi_uid: Option<PiUserId>,
  pub pi_token: Option<String>,
  pub paymentid: String,
  pub txid: String,
  pub object_id: Option<Uuid>,
  pub comment: Option<String>,
  pub auth: Option<String>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct PiPaymentCompleteResponse {
  pub id: PiPaymentId,
  pub paymentid: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct PiKey {
  pub domain: Option<String>,  
  pub pi_username: String,
  pub pi_uid: Option<PiUserId>,
  pub pi_token: Option<String>,
  pub pi_key: Option<String>,
  pub auth: Option<String>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct PiKeyResponse {
  pub success: bool,
  pub id: Option<Uuid>,
}

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct PiPaymentArgs {
  pub amount: f64,
  pub memo: Option<String>,
  pub metadata: Option<Value>,
  pub uid: String,
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
  pub from_address: String,
  pub to_address: String,
  pub direction: String,
  pub created_at: String,
  pub network: String,
  pub status: PiPaymentStatus,
  pub transaction: Option<PiPaymentTransaction>,
  pub metadata: Option<Value>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TxRequest {
  pub txid: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct GetPiBalances {
  pub domain: Option<String>,  
  pub auth: Sensitive<String>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct GetPiBalancesResponse {
  pub status: String,
  pub total: f64,
  pub withdrawed: f64,
  pub amount: f64,
  pub pending: f64,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CreatePayment {
  pub domain: Option<String>,
  pub obj_cat: Option<String>,
  pub obj_id: Option<Uuid>,
  pub comment: Option<String>,
  pub network: Option<String>,
  pub asset: Option<String>,
  pub amount: Option<f64>,
  pub inout: bool,
  pub memo: Option<String>,
  pub pi_token: Option<String>,
  pub pipaymentid: Option<String>,
  pub auth: Sensitive<String>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CreatePaymentResponse {
  pub success: bool,
  pub id: PiPaymentId,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct PiWithdraw {
  pub domain: Option<String>,  
  pub amount: f64,
  pub comment: Option<String>,
  pub auth: Sensitive<String>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct PiWithdrawResponse {
  pub id: Option<PiPaymentId>,
  pub status: String,
  pub paymentid: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct GetPayment {
  pub id: PiPaymentId,
  pub auth: Sensitive<String>,
}

#[derive(Serialize, Debug)]
pub struct GetPaymentResponse {
  //pub pid: String,
  //pub dto: PiPaymentDto,
  pub payment: Option<PiPaymentSafe>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct GetPayments {
  pub sort: Option<String>,
  pub sent: Option<bool>,
  pub page: Option<i64>,
  pub limit: Option<i64>,
  pub auth: Sensitive<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetPaymentsResponse {
  pub pipayments: Vec<PiPaymentSafe>,
}


