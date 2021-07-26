use crate::{schema::pipayment, PaymentId, PersonId, PiUserId};
//use diesel::sql_types::Jsonb;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

//#[changeset_options(treat_none_as_null = "true")]
#[derive(Deserialize, Clone, Queryable, Identifiable, PartialEq, Debug, Serialize)]
#[table_name = "pipayment"]
pub struct PiPayment {
  pub id: PaymentId,
  pub person_id: Option<PersonId>,
  pub ref_id: Option<Uuid>,
  pub testnet: bool,
  pub finished: bool,
  pub published: chrono::NaiveDateTime,
  pub updated: Option<chrono::NaiveDateTime>,
  pub comment: Option<String>,

  pub pi_uid: Option<PiUserId>,
  pub pi_username: String,

  pub identifier: String,
  pub user_uid: String,
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
  pub metadata: Option<Value>,
  pub extras: Option<Value>,
}

#[derive(Insertable, AsChangeset, Clone)]
#[table_name = "pipayment"]
pub struct PiPaymentForm {
  pub person_id: Option<PersonId>,
  pub ref_id: Option<Uuid>,
  pub testnet: bool,
  pub finished: bool,
  pub updated: Option<chrono::NaiveDateTime>,
  pub comment: Option<String>,

  pub pi_uid: Option<PiUserId>,
  pub pi_username: String,
  pub identifier: String,
  pub user_uid: String,
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
  pub metadata: Option<Value>,
  pub extras: Option<Value>,
}
