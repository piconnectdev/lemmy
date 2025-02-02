use crate::newtypes::{InstanceId, FederationAllowListId};
#[cfg(feature = "full")]
use crate::schema::federation_allowlist;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "full", derive(Queryable, Associations, Identifiable))]
#[cfg_attr(
  feature = "full",
  diesel(belongs_to(crate::source::instance::Instance))
)]
#[cfg_attr(feature = "full", diesel(table_name = federation_allowlist))]
pub struct FederationAllowList {
  pub id: FederationAllowListId,
  pub instance_id: InstanceId,
  pub published: chrono::NaiveDateTime,
  pub updated: Option<chrono::NaiveDateTime>,
}

#[derive(Clone, Default)]
#[cfg_attr(feature = "full", derive(Insertable, AsChangeset))]
#[cfg_attr(feature = "full", diesel(table_name = federation_allowlist))]
pub struct FederationAllowListForm {
  pub instance_id: InstanceId,
  pub updated: Option<chrono::NaiveDateTime>,
}
