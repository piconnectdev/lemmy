#[macro_use]
extern crate diesel;

#[macro_use]
extern crate diesel_derive_newtype;

use chrono::NaiveDateTime;
use diesel::{
  backend::Backend,
  deserialize::FromSql,
  serialize::{Output, ToSql},
  sql_types::Text,
};
use serde::{Deserialize, Serialize};
use std::{
  fmt,
  fmt::{Display, Formatter},
  io::Write,
};
use url::Url;
use uuid::Uuid;

pub mod schema;
pub mod source;

#[derive(
  Debug, Copy, Clone, Hash, Eq, PartialEq, Default, Serialize, Deserialize, DieselNewType,
)]
pub struct PostId(pub Uuid);

impl fmt::Display for PostId {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

#[derive(
  Debug, Copy, Clone, Hash, Eq, PartialEq, Default, Serialize, Deserialize, DieselNewType,
)]
pub struct PersonId(pub Uuid);

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Serialize, Deserialize, DieselNewType)]
pub struct CommentId(pub Uuid);

impl fmt::Display for CommentId {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

#[derive(
  Debug, Copy, Clone, Hash, Eq, PartialEq, Default, Serialize, Deserialize, DieselNewType,
)]
pub struct CommunityId(pub Uuid);

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Serialize, Deserialize, DieselNewType)]
pub struct LocalUserId(pub Uuid);

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Serialize, Deserialize, DieselNewType)]
pub struct PrivateMessageId(Uuid);

impl fmt::Display for PrivateMessageId {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Serialize, Deserialize, DieselNewType)]
pub struct PersonMentionId(i64);

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Serialize, Deserialize, DieselNewType)]
pub struct ActivityId(Uuid);

impl fmt::Display for ActivityId {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Serialize, Deserialize, DieselNewType)]
pub struct PostLikeId(i64);

impl fmt::Display for PostLikeId {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Serialize, Deserialize, DieselNewType)]
pub struct PostSaveId(i32);

impl fmt::Display for PostSaveId {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Serialize, Deserialize, DieselNewType)]
pub struct PostReadId(i64);

impl fmt::Display for PostReadId {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Serialize, Deserialize, DieselNewType)]
pub struct PostReportId(i64);

impl fmt::Display for PostReportId {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Serialize, Deserialize, DieselNewType)]
pub struct CommentReportId(i64);

impl fmt::Display for CommentReportId {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Serialize, Deserialize, DieselNewType)]
pub struct CommentLikeId(i64);

impl fmt::Display for CommentLikeId {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Serialize, Deserialize, DieselNewType)]
pub struct CommentSavedId(i32);

impl fmt::Display for CommentSavedId {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Serialize, Deserialize, DieselNewType)]
pub struct CommentAggregatesId(i64);

impl fmt::Display for CommentAggregatesId {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Serialize, Deserialize, DieselNewType)]
pub struct CommunityFollowerId(i64);

impl fmt::Display for CommunityFollowerId {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Serialize, Deserialize, DieselNewType)]
pub struct CommunityModeratorId(Uuid);

impl fmt::Display for CommunityModeratorId {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Serialize, Deserialize, DieselNewType)]
pub struct CommunityPersonBanId(i32);

impl fmt::Display for CommunityPersonBanId {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Serialize, Deserialize, DieselNewType)]
pub struct CommunityAggregatesId(i64);

impl fmt::Display for CommunityAggregatesId {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Serialize, Deserialize, DieselNewType)]
pub struct PersonAggregatesId(i64);

impl fmt::Display for PersonAggregatesId {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Serialize, Deserialize, DieselNewType)]
pub struct PostAggregatesId(i64);

impl fmt::Display for PostAggregatesId {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Serialize, Deserialize, DieselNewType)]
pub struct SiteAggregatesId(i64);

impl fmt::Display for SiteAggregatesId {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Serialize, Deserialize, DieselNewType)]
pub struct ModRemovePostId(i64);

impl fmt::Display for ModRemovePostId {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

// #[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Serialize, Deserialize, DieselNewType)]
// pub struct ModRemovePostId(i64);

// impl fmt::Display for ModRemovePostId {
//   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//     write!(f, "{}", self.0)
//   }
// }

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Serialize, Deserialize, DieselNewType)]
pub struct ModLockPostId(i64);

impl fmt::Display for ModLockPostId {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Serialize, Deserialize, DieselNewType)]
pub struct ModStickyPostId(i64);

impl fmt::Display for ModStickyPostId {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Serialize, Deserialize, DieselNewType)]
pub struct ModRemoveCommentId(i64);

impl fmt::Display for ModRemoveCommentId {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Serialize, Deserialize, DieselNewType)]
pub struct ModRemoveCommunityId(i64);

impl fmt::Display for ModRemoveCommunityId {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Serialize, Deserialize, DieselNewType)]
pub struct ModBanFromCommunityId(i64);

impl fmt::Display for ModBanFromCommunityId {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Serialize, Deserialize, DieselNewType)]
pub struct ModAddCommunityId(i64);

impl fmt::Display for ModAddCommunityId {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Serialize, Deserialize, DieselNewType)]
pub struct ModBanId(i64);

impl fmt::Display for ModBanId {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Serialize, Deserialize, DieselNewType)]
pub struct ModAddId(i64);

impl fmt::Display for ModAddId {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

/*
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Serialize, Deserialize, DieselNewType)]
pub struct CommonId(i64);

impl fmt::Display for CommonId {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}
*/

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Serialize, Deserialize, DieselNewType)]
pub struct PaymentId(i64);

impl fmt::Display for PaymentId {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Serialize, Deserialize, DieselNewType)]
pub struct PiPaymentId(uuid::Uuid);

impl fmt::Display for PiPaymentId {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Serialize, Deserialize, DieselNewType)]
pub struct PiUserId(Uuid);

impl fmt::Display for PiUserId {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

#[repr(transparent)]
#[derive(Clone, PartialEq, Serialize, Deserialize, Debug, AsExpression, FromSqlRow)]
#[sql_type = "Text"]
pub struct DbUrl(Url);

impl<DB: Backend> ToSql<Text, DB> for DbUrl
where
  String: ToSql<Text, DB>,
{
  fn to_sql<W: Write>(&self, out: &mut Output<W, DB>) -> diesel::serialize::Result {
    self.0.to_string().to_sql(out)
  }
}

impl<DB: Backend> FromSql<Text, DB> for DbUrl
where
  String: FromSql<Text, DB>,
{
  fn from_sql(bytes: Option<&DB::RawValue>) -> diesel::deserialize::Result<Self> {
    let str = String::from_sql(bytes)?;
    Ok(DbUrl(Url::parse(&str)?))
  }
}

impl DbUrl {
  pub fn into_inner(self) -> Url {
    self.0
  }
}

impl Display for DbUrl {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    self.to_owned().into_inner().fmt(f)
  }
}

impl From<DbUrl> for Url {
  fn from(url: DbUrl) -> Self {
    url.0
  }
}

impl From<Url> for DbUrl {
  fn from(url: Url) -> Self {
    DbUrl(url)
  }
}

// TODO: can probably move this back to lemmy_db_queries
pub fn naive_now() -> NaiveDateTime {
  chrono::prelude::Utc::now().naive_utc()
}
