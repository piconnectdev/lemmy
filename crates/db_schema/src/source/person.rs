use crate::newtypes::{DbUrl, InstanceId, PersonId, PersonFollowerId, CommunityId};
#[cfg(feature = "full")]
use crate::schema::{person, person_follower};
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "full", derive(Queryable, Identifiable))]
#[cfg_attr(feature = "full", diesel(table_name = person))]
pub struct Person {
  pub id: PersonId,
  pub name: String,
  pub display_name: Option<String>,
  pub avatar: Option<DbUrl>,
  pub banned: bool,
  pub published: chrono::NaiveDateTime,
  pub updated: Option<chrono::NaiveDateTime>,
  pub actor_id: DbUrl,
  pub bio: Option<String>,
  pub local: bool,
  pub private_key: Option<String>,
  pub public_key: String,
  pub last_refreshed_at: chrono::NaiveDateTime,
  pub banner: Option<DbUrl>,
  pub deleted: bool,
  pub inbox_url: DbUrl,
  pub shared_inbox_url: Option<DbUrl>,
  pub matrix_user_id: Option<String>,
  pub admin: bool,
  pub bot_account: bool,
  pub ban_expires: Option<chrono::NaiveDateTime>,
  pub instance_id: InstanceId,
  pub home: Option<CommunityId>,
  pub external_id: Option<String>,
  pub external_name: Option<String>,
  pub verified: bool,
  //pub private_seeds: Option<String>,
  pub pi_address: Option<String>,
  pub web3_address: Option<String>,
  pub pol_address: Option<String>,
  pub dap_address: Option<String>,
  pub cosmos_address: Option<String>,
  pub sui_address: Option<String>,
  pub auth_sign: Option<String>,
  pub srv_sign: Option<String>,
  pub pipayid: Option<String>,
  pub tx : Option<String>,
  //pub extras: Option<Value>,
}

/// A safe representation of person, without the sensitive info
#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "full", derive(Queryable, Identifiable))]
#[cfg_attr(feature = "full", diesel(table_name = person))]
pub struct PersonSafe {
  pub id: PersonId,
  pub name: String,
  pub display_name: Option<String>,
  pub avatar: Option<DbUrl>,
  pub banned: bool,
  pub published: chrono::NaiveDateTime,
  pub updated: Option<chrono::NaiveDateTime>,
  pub actor_id: DbUrl,
  pub bio: Option<String>,
  pub local: bool,
  pub banner: Option<DbUrl>,
  pub deleted: bool,
  pub inbox_url: DbUrl,
  pub shared_inbox_url: Option<DbUrl>,
  pub matrix_user_id: Option<String>,
  pub admin: bool,
  pub bot_account: bool,
  pub ban_expires: Option<chrono::NaiveDateTime>,
  pub instance_id: InstanceId,
  pub home: Option<CommunityId>,
  //pub external_id: Option<String>,
  //pub external_name: Option<String>,
  pub verified: bool,
  pub pi_address: Option<String>,
  pub web3_address: Option<String>,
  pub pol_address: Option<String>,
  pub dap_address: Option<String>,
  pub cosmos_address: Option<String>,
  pub sui_address: Option<String>,
  pub auth_sign: Option<String>,
  pub srv_sign: Option<String>,
  pub tx : Option<String>,
}

#[derive(Clone, TypedBuilder)]
#[builder(field_defaults(default))]
#[cfg_attr(feature = "full", derive(Insertable, AsChangeset))]
#[cfg_attr(feature = "full", diesel(table_name = person))]
pub struct PersonInsertForm {
  #[builder(!default)]
  pub name: String,
  #[builder(!default)]
  pub public_key: String,
  #[builder(!default)]
  pub instance_id: InstanceId,
  pub display_name: Option<String>,
  pub avatar: Option<DbUrl>,
  pub banned: Option<bool>,
  pub published: Option<chrono::NaiveDateTime>,
  pub updated: Option<chrono::NaiveDateTime>,
  pub actor_id: Option<DbUrl>,
  pub bio: Option<String>,
  pub local: Option<bool>,
  pub private_key: Option<String>,
  pub last_refreshed_at: Option<chrono::NaiveDateTime>,
  pub banner: Option<DbUrl>,
  pub deleted: Option<bool>,
  pub inbox_url: Option<DbUrl>,
  pub shared_inbox_url: Option<DbUrl>,
  pub matrix_user_id: Option<String>,
  pub admin: Option<bool>,
  pub bot_account: Option<bool>,
  pub ban_expires: Option<chrono::NaiveDateTime>,
  pub home: Option<CommunityId>,
  pub external_id: Option<String>,
  pub external_name: Option<String>,
  pub verified: bool,
  pub pi_address: Option<Option<String>>,
  pub web3_address: Option<Option<String>>,
  pub pol_address: Option<Option<String>>,
  pub dap_address: Option<Option<String>>,
  pub cosmos_address: Option<Option<String>>,
  pub sui_address: Option<Option<String>>,
  pub auth_sign: Option<Option<String>>,
  pub srv_sign: Option<Option<String>>,
  pub tx : Option<Option<String>>,
}

#[derive(Clone, TypedBuilder)]
#[cfg_attr(feature = "full", derive(AsChangeset))]
#[cfg_attr(feature = "full", diesel(table_name = person))]
#[builder(field_defaults(default))]
pub struct PersonUpdateForm {
  pub display_name: Option<Option<String>>,
  pub avatar: Option<Option<DbUrl>>,
  pub banned: Option<bool>,
  pub updated: Option<Option<chrono::NaiveDateTime>>,
  pub actor_id: Option<DbUrl>,
  pub bio: Option<Option<String>>,
  pub local: Option<bool>,
  pub public_key: Option<String>,
  pub private_key: Option<Option<String>>,
  pub last_refreshed_at: Option<chrono::NaiveDateTime>,
  pub banner: Option<Option<DbUrl>>,
  pub deleted: Option<bool>,
  pub inbox_url: Option<DbUrl>,
  pub shared_inbox_url: Option<Option<DbUrl>>,
  pub matrix_user_id: Option<Option<String>>,
  pub admin: Option<bool>,
  pub bot_account: Option<bool>,
  pub ban_expires: Option<Option<chrono::NaiveDateTime>>,
  //pub external_id: Option<String>,
  //pub verified: bool,
  //pub private_seeds: Option<Option<String>>,
  pub pi_address: Option<Option<String>>,
  pub web3_address: Option<Option<String>>,
  pub pol_address: Option<Option<String>>,
  pub dap_address: Option<Option<String>>,
  pub cosmos_address: Option<Option<String>>,
  pub sui_address: Option<Option<String>>,
  pub auth_sign: Option<Option<String>>,
  pub srv_sign: Option<Option<String>>,
  pub tx : Option<Option<String>>,
}

#[derive(PartialEq, Eq, Debug)]
#[cfg_attr(feature = "full", derive(Identifiable, Queryable, Associations))]
#[cfg_attr(feature = "full", diesel(belongs_to(crate::source::person::Person)))]
#[cfg_attr(feature = "full", diesel(table_name = person_follower))]
pub struct PersonFollower {
  pub id: PersonFollowerId,
  pub person_id: PersonId,
  pub follower_id: PersonId,
  pub published: chrono::NaiveDateTime,
  pub pending: bool,
}

#[derive(Clone)]
#[cfg_attr(feature = "full", derive(Insertable, AsChangeset))]
#[cfg_attr(feature = "full", diesel(table_name = person_follower))]
pub struct PersonFollowerForm {
  pub person_id: PersonId,
  pub follower_id: PersonId,
  pub pending: bool,
}
