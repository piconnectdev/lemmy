use crate::newtypes::{LocalSiteRateLimitId, LocalSiteId};
#[cfg(feature = "full")]
use crate::schema::local_site_rate_limit;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(PartialEq, Eq, Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "full", derive(Queryable, Identifiable))]
#[cfg_attr(feature = "full", diesel(table_name = local_site_rate_limit))]
#[cfg_attr(
  feature = "full",
  diesel(belongs_to(crate::source::local_site::LocalSite))
)]
pub struct LocalSiteRateLimit {
  pub id: LocalSiteRateLimitId,
  pub local_site_id: LocalSiteId,
  pub message: i32,
  pub message_per_second: i32,
  pub post: i32,
  pub post_per_second: i32,
  pub register: i32,
  pub register_per_second: i32,
  pub image: i32,
  pub image_per_second: i32,
  pub comment: i32,
  pub comment_per_second: i32,
  pub search: i32,
  pub search_per_second: i32,
  pub published: chrono::NaiveDateTime,
  pub updated: Option<chrono::NaiveDateTime>,
}

#[derive(Clone, TypedBuilder)]
#[builder(field_defaults(default))]
#[cfg_attr(feature = "full", derive(Insertable))]
#[cfg_attr(feature = "full", diesel(table_name = local_site_rate_limit))]
pub struct LocalSiteRateLimitInsertForm {
  #[builder(!default)]
  pub local_site_id: LocalSiteId,
  pub message: Option<i32>,
  pub message_per_second: Option<i32>,
  pub post: Option<i32>,
  pub post_per_second: Option<i32>,
  pub register: Option<i32>,
  pub register_per_second: Option<i32>,
  pub image: Option<i32>,
  pub image_per_second: Option<i32>,
  pub comment: Option<i32>,
  pub comment_per_second: Option<i32>,
  pub search: Option<i32>,
  pub search_per_second: Option<i32>,
}

#[derive(Clone, TypedBuilder)]
#[builder(field_defaults(default))]
#[cfg_attr(feature = "full", derive(AsChangeset))]
#[cfg_attr(feature = "full", diesel(table_name = local_site_rate_limit))]
pub struct LocalSiteRateLimitUpdateForm {
  pub message: Option<i32>,
  pub message_per_second: Option<i32>,
  pub post: Option<i32>,
  pub post_per_second: Option<i32>,
  pub register: Option<i32>,
  pub register_per_second: Option<i32>,
  pub image: Option<i32>,
  pub image_per_second: Option<i32>,
  pub comment: Option<i32>,
  pub comment_per_second: Option<i32>,
  pub search: Option<i32>,
  pub search_per_second: Option<i32>,
  pub updated: Option<Option<chrono::NaiveDateTime>>,
}
