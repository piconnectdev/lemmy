use crate::newtypes::{PersonId, PersonBalanceId};
#[cfg(feature = "full")]
use crate::schema::person_balance;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use typed_builder::TypedBuilder;

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "full", derive(Queryable, Identifiable))]
#[cfg_attr(feature = "full", diesel(table_name = person_balance))]
pub struct PersonBalance {
  pub id: PersonBalanceId,
  pub person_id: PersonId,
  pub published: chrono::NaiveDateTime,
  pub asset: Option<String>,
  pub deposited: f64,
  pub rewarded: f64,
  pub withdrawed: f64,
  pub amount: f64,
  pub pending: f64,
  pub extras: Option<Value>,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "full", derive(Queryable, Identifiable))]
#[cfg_attr(feature = "full", diesel(table_name = person_balance))]
pub struct PersonBalanceSafe {
  pub id: PersonBalanceId,
  pub person_id: PersonId,
  pub published: chrono::NaiveDateTime,
  pub asset: Option<String>,
  pub deposited: f64,
  pub rewarded: f64,
  pub withdrawed: f64,
  pub amount: f64,
  pub pending: f64,
  pub extras: Option<Value>,
}

#[derive(Debug, Clone, TypedBuilder)]
#[builder(field_defaults(default))]
#[cfg_attr(feature = "full", derive(Insertable, AsChangeset))]
#[cfg_attr(feature = "full", diesel(table_name = person_balance))]
pub struct PersonBalanceInsertForm {
  pub person_id: PersonId,
  pub asset: Option<String>,
  pub deposited: f64,
  pub rewarded: f64,
  pub amount: f64,
  pub withdrawed: f64,
  pub pending: f64,
}

#[cfg_attr(feature = "full", derive(AsChangeset))]
#[cfg_attr(feature = "full", diesel(table_name = person_balance))]
pub struct PersonBalanceUpdateForm {
  pub deposited: f64,
  pub rewarded: f64,
  pub withdrawed: f64,
  pub amount: f64,
  pub pending: f64,
}
