use crate::{newtypes::{PersonId, PiPaymentId, PiUserId}};
#[cfg(feature = "full")]
use crate::schema::pipayment;
//use diesel::sql_types::Jsonb;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;
use serde_json::Value;
use uuid::Uuid;
use std::fmt::Debug;

//#[changeset_options(treat_none_as_null = "true")]

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, TypedBuilder)]
#[cfg_attr(feature = "full", derive(Queryable, Identifiable))]
#[cfg_attr(feature = "full", diesel(table_name = pipayment))]
pub struct PiPayment {
  pub id: PiPaymentId,
  pub domain: Option<String>,
  pub instance_id: Option<PersonId>,
  pub person_id: Option<PersonId>,
  pub obj_cat: Option<String>,
  pub obj_id: Option<Uuid>,
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

#[derive(Debug, Clone, TypedBuilder)]
#[builder(field_defaults(default))]
#[cfg_attr(feature = "full", derive(Insertable, AsChangeset))]
#[cfg_attr(feature = "full", diesel(table_name = pipayment))]
pub struct PiPaymentInsertForm {
  // pub id: PiPaymentId,
  pub domain: Option<String>,
  pub instance_id: Option<PersonId>,
  pub person_id: Option<PersonId>,
  pub obj_cat: Option<String>,
  pub obj_id: Option<Uuid>,
  pub testnet: bool,
  pub finished: bool,
  pub updated: Option<chrono::NaiveDateTime>,
  pub comment: Option<String>,
  pub ref_id: Option<Uuid>,

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

#[derive(Debug, Clone, TypedBuilder)]
#[builder(field_defaults(default))]
#[cfg_attr(feature = "full", derive(AsChangeset))]
#[cfg_attr(feature = "full", diesel(table_name = pipayment))]
pub struct PiPaymentUpdateForm {
  // pub id: PiPaymentId,
  pub person_id: Option<PersonId>,
  //pub testnet: bool,
  pub finished: bool,
  pub updated: Option<chrono::NaiveDateTime>,
  //pub comment: Option<String>,
  //pub ref_id: Option<Uuid>,

  //pub pi_uid: Option<PiUserId>,
  //pub pi_username: String,
  //pub identifier: String,
  //pub user_uid: String,
  //pub amount: f64,
  //pub memo: String,
  //pub to_address: String,
  //pub created_at: Option<chrono::NaiveDateTime>,

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
