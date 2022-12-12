use crate::{newtypes::{PersonId, PiPaymentId, PiUserId, InstanceId}};
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
  pub instance_id: Option<InstanceId>,
  pub person_id: Option<PersonId>,
  pub testnet: bool,
  pub published: chrono::NaiveDateTime,
  pub object_cat: Option<String>,
  pub object_id: Option<Uuid>,

  pub pi_username: String,
  pub pi_uid: Option<PiUserId>,
  pub finished: bool,
  pub updated: Option<chrono::NaiveDateTime>,
  pub other_id: Option<Uuid>,

  pub identifier: String,
  pub user_uid: String,
  pub amount: f64,
  pub memo: String, // register, tip:page, page, note, message
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
  pub notes: Option<String>, // username, post_id, comment_id
}

/// A safe representation of payment, without the sensitive info

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "full", derive(Queryable, Identifiable))]
#[cfg_attr(feature = "full", diesel(table_name = pipayment))]
pub struct PiPaymentSafe {
  pub id: PiPaymentId,
  pub domain: Option<String>,
  pub instance_id: Option<InstanceId>,
  pub person_id: Option<PersonId>,
  pub testnet: bool,
  pub published: chrono::NaiveDateTime,
  pub object_cat: Option<String>,
  pub object_id: Option<Uuid>,

  pub pi_username: String,
  pub pi_uid: Option<PiUserId>,
  pub finished: bool,
  pub updated: Option<chrono::NaiveDateTime>,
  pub other_id: Option<Uuid>, //Receiptor id

  pub identifier: String,
  pub user_uid: String,
  pub amount: f64,
  pub memo: String, // register, tip:page, page, note, message
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
  pub notes: Option<String>, // username, post_id, comment_id

}

#[derive(Debug, Clone, TypedBuilder)]
#[builder(field_defaults(default))]
#[cfg_attr(feature = "full", derive(Insertable, AsChangeset))]
#[cfg_attr(feature = "full", diesel(table_name = pipayment))]
pub struct PiPaymentInsertForm {
  // pub id: PiPaymentId,
  pub domain: Option<String>,
  pub instance_id: Option<InstanceId>,
  pub person_id: Option<PersonId>,
  pub testnet: bool,
  pub object_cat: Option<String>,
  pub object_id: Option<Uuid>,
  
  pub pi_username: String,
  pub pi_uid: Option<PiUserId>,
  pub finished: bool,
  pub updated: Option<chrono::NaiveDateTime>,
  pub other_id: Option<Uuid>,

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
  pub notes: Option<String>,

}

#[derive(Debug, Clone, TypedBuilder)]
#[builder(field_defaults(default))]
#[cfg_attr(feature = "full", derive(AsChangeset))]
#[cfg_attr(feature = "full", diesel(table_name = pipayment))]
pub struct PiPaymentUpdateForm {
  //pub id: PiPaymentId,
  //pub person_id: Option<PersonId>,
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
  pub notes: Option<String>,
}
