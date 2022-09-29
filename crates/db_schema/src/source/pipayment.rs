use crate::{newtypes::{PersonId, PaymentId, PiUserId}, schema::{pipayment}};
//use crate::schema::pipayment;
//use diesel::sql_types::Jsonb;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;
use std::fmt::Debug;

//#[cfg(feature = "full")]
//use crate::schema::{pipayment};

//#[changeset_options(treat_none_as_null = "true")]
//#[derive(Default)]
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "full", derive(Queryable, Identifiable))]
#[diesel(table_name = pipayment)]
pub struct PiPayment {
  pub id: PaymentId,
  pub person_id: Option<PersonId>,
  pub ref_id: Option<Uuid>, //Receiptor id
  pub testnet: bool,
  pub finished: bool,
  pub published: chrono::NaiveDateTime,
  pub updated: Option<chrono::NaiveDateTime>,
  pub comment: Option<String>, // username, post_id, comment_id

  pub pi_uid: Option<PiUserId>,
  pub pi_username: String,

  pub identifier: String,
  pub user_uid: String,
  pub amount: f64,
  pub memo: String, // wepi:account, wepi:tip, wepi:post, wepi:comment
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

//#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
//#[cfg_attr(feature = "full", derive(Queryable))]
//#[derive(Default)]
//#[cfg_attr(feature = "full", derive(Insertable, AsChangeset))]
//#[cfg_attr(feature = "full", table_name = "pipayment")]
#[derive(Insertable, AsChangeset)]
#[diesel(table_name = pipayment)]
pub struct PiPaymentForm {
  // pub id: PaymentId,
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
