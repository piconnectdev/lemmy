use crate::{schema::pipayment, DbUrl, PaymentId, PersonId, PiPaymentId, PiUserId};
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
  pub person_id: PersonId,
  pub payment_id: PiPaymentId,
  pub user_uid: Option<PiUserId>,
  pub person_name: String,
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
  pub payment_dto: Value,
}

#[derive(Insertable, AsChangeset, Clone)]
#[table_name = "pipayment"]
pub struct PaymentForm {
  pub person_id: PersonId,
  pub payment_id: PiPaymentId,
  pub user_uid: Option<PiUserId>,
  pub person_name: String,
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
  pub payment_dto: Value,
}

// TODO redo these, check table defaults
/*
#[derive(Insertable, AsChangeset, Clone, Default)]
#[table_name = "local_user"]
pub struct LocalUserForm {
  pub person_id: PersonId,
  pub password_encrypted: String,
  pub email: Option<Option<String>>,
  pub show_nsfw: Option<bool>,
  pub theme: Option<String>,
  pub default_sort_type: Option<i16>,
  pub default_listing_type: Option<i16>,
  pub lang: Option<String>,
  pub show_avatars: Option<bool>,
  pub send_notifications_to_email: Option<bool>,
  pub show_bot_accounts: Option<bool>,
  pub show_scores: Option<bool>,
  pub show_read_posts: Option<bool>,
}

/// A local user view that removes password encrypted
#[derive(Clone, Queryable, Identifiable, PartialEq, Debug, Serialize)]
#[table_name = "local_user"]
pub struct LocalUserSettings {
  pub id: LocalUserId,
  pub person_id: PersonId,
  pub email: Option<String>,
  pub show_nsfw: bool,
  pub theme: String,
  pub default_sort_type: i16,
  pub default_listing_type: i16,
  pub lang: String,
  pub show_avatars: bool,
  pub send_notifications_to_email: bool,
  pub validator_time: chrono::NaiveDateTime,
  pub show_bot_accounts: bool,
  pub show_scores: bool,
  pub show_read_posts: bool,
}
*/
