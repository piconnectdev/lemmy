use crate::{newtypes::{DbUrl, ActivityId}, schema::activity, };
use serde_json::Value;
use std::fmt::Debug;

#[derive(PartialEq, Debug, Queryable, Identifiable)]
#[table_name = "activity"]
pub struct Activity {
  pub id: ActivityId,
  pub data: Value,
  pub local: bool,
  pub published: chrono::NaiveDateTime,
  pub updated: Option<chrono::NaiveDateTime>,
  pub ap_id: DbUrl,
  pub sensitive: Option<bool>,
}

#[derive(Insertable, AsChangeset)]
#[table_name = "activity"]
pub struct ActivityForm {
  pub data: Value,
  pub local: Option<bool>,
  pub updated: Option<chrono::NaiveDateTime>,
  pub ap_id: DbUrl,
  pub sensitive: bool,
}
