use crate::newtypes::*;
#[cfg(feature = "full")]
use crate::schema::pipayment;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt::Debug;
use typed_builder::TypedBuilder;
use uuid::Uuid;

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, TypedBuilder)]
#[cfg_attr(feature = "full", derive(Queryable, Identifiable))]
#[cfg_attr(feature = "full", diesel(table_name = pipayment))]
pub struct PiPayment {
  pub id: PiPaymentId,
  pub domain: Option<String>,
  pub instance_id: Option<InstanceId>,
  pub person_id: Option<PersonId>,
  pub obj_cat: Option<String>,
  pub obj_id: Option<Uuid>,
  pub a2u: i32,
  pub step: i32,
  pub asset: Option<String>,
  pub fee: f64,
  pub testnet: bool,
  pub finished: bool,
  pub published: chrono::NaiveDateTime,
  pub updated: Option<chrono::NaiveDateTime>,
  pub ref_id: Option<Uuid>,    //Receiptor id
  pub comment: Option<String>, // username, post_id, comment_id
  pub stat: Option<String>,

  pub pi_uid: Option<PiUserId>,
  pub pi_username: String,

  pub identifier: Option<String>,
  pub user_uid: Option<String>,
  pub amount: f64,
  pub memo: Option<String>, // account, tip, page, note, withdraw
  pub from_address: Option<String>,
  pub to_address: Option<String>,
  pub direction: Option<String>,
  pub created_at: Option<chrono::NaiveDateTime>,

  pub approved: bool,
  pub verified: bool,
  pub completed: bool,
  pub cancelled: bool,
  pub user_cancelled: bool,
  pub tx_verified: bool,
  pub tx_link: Option<String>,
  pub tx_id: Option<String>,
  pub network: Option<String>,
  pub metadata: Option<Value>,
  pub extras: Option<Value>,
}

/*
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, TypedBuilder)]
#[cfg_attr(feature = "full", derive(Queryable, Identifiable))]
#[cfg_attr(feature = "full", diesel(table_name = pipayment))]
pub struct PiPaymentSafe {
  pub id: PiPaymentId,
  //pub domain: Option<String>,
  //pub instance_id: Option<PersonId>,
  pub person_id: Option<PersonId>,
  pub obj_cat: Option<String>,
  pub obj_id: Option<Uuid>,
  pub a2u: i32,
  pub step: i32,
  pub asset: Option<String>,
  pub fee: f64,
  pub testnet: bool,
  pub finished: bool,
  pub published: chrono::NaiveDateTime,
  pub updated: Option<chrono::NaiveDateTime>,
  pub ref_id: Option<Uuid>,    //Receiptor id
  pub comment: Option<String>, // username, post_id, comment_id
  pub stat: Option<String>,
  //pub pi_uid: Option<PiUserId>,
  //pub pi_username: String,
  pub identifier: Option<String>,
  pub user_uid: Option<String>,
  pub amount: f64,
  pub memo: Option<String>,
  pub from_address: Option<String>,
  pub to_address: Option<String>,
  pub direction: Option<String>,
  pub created_at: Option<chrono::NaiveDateTime>,

  pub approved: bool,
  pub verified: bool,
  pub completed: bool,
  pub cancelled: bool,
  pub user_cancelled: bool,
  pub tx_verified: bool,
  pub tx_link: Option<String>,
  pub tx_id: Option<String>,
  pub network: Option<String>,
  pub metadata: Option<Value>,
  pub extras: Option<Value>,
}
*/

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(default))]
#[cfg_attr(feature = "full", derive(Insertable, AsChangeset))]
#[cfg_attr(feature = "full", diesel(table_name = pipayment))]
pub struct PiPaymentInsertForm {
  pub domain: Option<String>,
  pub instance_id: Option<InstanceId>,
  pub person_id: Option<PersonId>,
  pub obj_cat: Option<String>,
  pub obj_id: Option<Uuid>,
  pub a2u: i32,
  pub step: i32,

  pub asset: Option<String>,
  pub fee: f64,

  pub testnet: bool,
  pub finished: bool,
  pub updated: Option<chrono::NaiveDateTime>,
  pub ref_id: Option<Uuid>,
  pub comment: Option<String>,

  pub pi_uid: Option<PiUserId>,
  pub pi_username: String,
  pub identifier: Option<String>,
  pub user_uid: Option<String>,
  pub amount: f64,
  pub memo: Option<String>,
  pub from_address: Option<String>,
  pub to_address: Option<String>,
  pub direction: Option<String>,
  pub created_at: Option<chrono::NaiveDateTime>,

  pub approved: bool,
  pub verified: bool,
  pub completed: bool,
  pub cancelled: bool,
  pub user_cancelled: bool,
  pub tx_verified: bool,
  pub tx_link: Option<String>,
  pub tx_id: Option<String>,
  pub network: Option<String>,
  pub metadata: Option<Value>,
  pub extras: Option<Value>,
}

#[derive(Debug, Clone, TypedBuilder)]
#[builder(field_defaults(default))]
#[cfg_attr(feature = "full", derive(AsChangeset))]
#[cfg_attr(feature = "full", diesel(table_name = pipayment))]
pub struct PiPaymentUpdateForm {
  //pub id: PiPaymentId,
  //pub person_id: Option<PersonId>,
  pub step: i32,
  //pub testnet: bool,
  pub finished: bool,
  pub updated: Option<chrono::NaiveDateTime>,
  //pub comment: Option<String>,
  //pub ref_id: Option<Uuid>,

  //pub pi_uid: Option<PiUserId>,
  //pub pi_username: String,
  pub identifier: Option<String>,
  pub user_uid: Option<String>,
  pub amount: f64,
  pub memo: Option<String>,
  pub from_address: Option<String>,
  pub to_address: Option<String>,
  pub direction: Option<String>,
  pub created_at: Option<chrono::NaiveDateTime>,

  pub approved: bool,
  pub verified: bool,
  pub completed: bool,
  pub cancelled: bool,
  pub user_cancelled: bool,
  pub tx_verified: bool,
  pub tx_link: Option<String>,
  pub tx_id: Option<String>,
  pub network: Option<String>,
  pub metadata: Option<Value>,
  pub extras: Option<Value>,
}

#[derive(Debug, Clone, TypedBuilder)]
#[builder(field_defaults(default))]
#[cfg_attr(feature = "full", derive(AsChangeset))]
#[cfg_attr(feature = "full", diesel(table_name = pipayment))]
pub struct PiPaymentUpdatePending {
  //pub id: PiPaymentId,
  //pub person_id: Option<PersonId>,
  pub step: i32,
  //pub testnet: bool,
  //pub finished: bool,
  pub updated: Option<chrono::NaiveDateTime>,
  //pub comment: Option<String>,
  //pub ref_id: Option<Uuid>,

  //pub pi_uid: Option<PiUserId>,
  //pub pi_username: String,
  pub identifier: Option<String>,
  //pub user_uid: Option<String>,
  //pub amount: f64,
  //pub memo: Option<String>,
  pub from_address: Option<String>,
  pub to_address: Option<String>,
  pub direction: Option<String>,
  pub created_at: Option<chrono::NaiveDateTime>,

  pub approved: bool,
  pub verified: bool,
  pub completed: bool,
  pub cancelled: bool,
  pub user_cancelled: bool,
  pub tx_verified: bool,
  pub tx_link: Option<String>,
  pub tx_id: Option<String>,
  pub network: Option<String>,
  //pub metadata: Option<Value>,
  //pub extras: Option<Value>,
}

#[derive(Debug, Clone, TypedBuilder)]
#[builder(field_defaults(default))]
#[cfg_attr(feature = "full", derive(AsChangeset))]
#[cfg_attr(feature = "full", diesel(table_name = pipayment))]
pub struct PiPaymentUpdateA2UForm {
  // pub id: PiPaymentId,
  // pub person_id: Option<PersonId>,
  pub step: i32,
  //pub testnet: bool,
  pub finished: bool,
  pub updated: Option<chrono::NaiveDateTime>,
  //pub comment: Option<String>,
  //pub ref_id: Option<Uuid>,
  pub pi_uid: Option<PiUserId>,
  //pub pi_username: String,
  pub identifier: String,
  pub user_uid: String,
  pub amount: f64,
  pub memo: String,
  pub from_address: String,
  pub to_address: String,
  pub direction: String,
  pub created_at: Option<chrono::NaiveDateTime>,

  pub approved: bool,
  pub verified: bool,
  pub completed: bool,
  pub cancelled: bool,
  pub user_cancelled: bool,
  pub tx_verified: bool,
  pub tx_link: String,
  pub tx_id: String,
  pub network: Option<String>,
  pub metadata: Option<Value>,
  pub extras: Option<Value>,
}
