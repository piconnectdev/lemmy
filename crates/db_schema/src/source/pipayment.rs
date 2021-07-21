use crate::{schema::pipayment, PaymentId, PersonId, PiPaymentId, PiUserId};
//use diesel::sql_types::Jsonb;
use serde::Serialize;
use serde_json::Value;
use uuid::Uuid;
/*
#[derive(FromSqlRow, AsExpression, serde::Serialize, serde::Deserialize, Debug, Default)]
#[sql_type = "Jsonb"]
pub struct PiPaymentStatus {
    pub developer_approved: bool,
    pub transaction_verified: bool,
    pub developer_completed: bool,
    pub cancelled: bool,
    pub user_cancelled: bool,
}

#[derive(FromSqlRow, AsExpression, serde::Serialize, serde::Deserialize, Debug, Default)]
#[sql_type = "Jsonb"]
pub struct PiPaymentTransaction {
    pub txid: Option<String>,
    pub verified: bool,
    pub link: String,
}

#[derive(FromSqlRow, AsExpression, serde::Serialize, serde::Deserialize, Debug, Default)]
#[sql_type = "Jsonb"]
pub struct PiPaymentDto {
    pub identifier: String,
    pub user_uid: String, /// uuid::Uuid
    pub amount: f64,
    pub memo: String,
    pub to_address: String,
    //pub created_at: chrono::NaiveDateTime,
    pub status: PiPaymentStatus,
    pub transaction: Option<PiPaymentTransaction>,
    pub metadata:	Option<Value>,
}
*/

//#[changeset_options(treat_none_as_null = "true")]
#[derive(Clone, Queryable, Identifiable, PartialEq, Debug, Serialize)]
#[table_name = "pipayment"]
pub struct Payment {
  pub id: PaymentId,
  pub person_id: Option<PersonId>,
  pub ref_id: Option<Uuid>,
  pub testnet: bool,
  pub published: chrono::NaiveDateTime,
  pub pi_payment_id: PiPaymentId,
  pub pi_uid: Option<PiUserId>,
  pub pi_username: String,
  pub identifier: String,
  pub amount: f64,
  pub memo: String,
  pub to_address: String,
  pub created_at: Option<chrono::NaiveDateTime>,
  pub approved: bool,
  pub verified: bool,
  pub completed: bool,
  pub cancelled: bool,
  pub user_cancelled: bool,
  pub tx_verified: bool,
  pub tx_link: String,
  pub tx_id: String,
  pub metadata: Value,
  pub payment_dto: Value,
}

#[derive(Insertable, AsChangeset, Clone)]
#[table_name = "pipayment"]
pub struct PaymentForm {
  pub person_id: Option<PersonId>,
  pub ref_id: Option<Uuid>,
  pub testnet: bool,
  pub pi_payment_id: PiPaymentId,
  pub pi_uid: Option<PiUserId>,
  pub pi_username: String,
  pub identifier: String,
  pub amount: f64,
  pub memo: String,
  pub to_address: String,
  pub created_at: Option<chrono::NaiveDateTime>,
  pub approved: bool,
  pub verified: bool,
  pub completed: bool,
  pub cancelled: bool,
  pub user_cancelled: bool,
  pub tx_verified: bool,
  pub tx_link: String,
  pub tx_id: String,
  pub metadata: Value,
  pub dto: Value,
}
