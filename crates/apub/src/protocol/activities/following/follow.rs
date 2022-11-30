use crate::{fetcher::user_or_community::UserOrCommunity, objects::person::ApubPerson};
use activitypub_federation::core::object_id::ObjectId;
use activitystreams_kinds::activity::FollowType;
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Follow {
  pub(crate) actor: ObjectId<ApubPerson>,
  pub(crate) object: ObjectId<UserOrCommunity>,
  #[serde(rename = "type")]
  pub(crate) kind: FollowType,
  pub(crate) id: Url,
}
