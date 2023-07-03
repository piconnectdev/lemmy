#[cfg(feature = "full")]
use activitypub_federation::{
  fetch::collection_id::CollectionId, fetch::object_id::ObjectId, traits::Collection,
  traits::Object,
};
#[cfg(feature = "full")]
use diesel_ltree::Ltree;
use serde::{Deserialize, Serialize};
use std::{
  fmt,
  fmt::{Display, Formatter},
  ops::Deref,
};
#[cfg(feature = "full")]
use ts_rs::TS;
use url::Url;
use uuid::Uuid;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "full", derive(DieselNewType, TS))]
#[cfg_attr(feature = "full", ts(export))]
/// The post id.
pub struct PostId(pub Uuid);
impl fmt::Display for PostId {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "full", derive(DieselNewType, TS))]
#[cfg_attr(feature = "full", ts(export))]
/// The person id.
pub struct PersonId(pub Uuid);

impl fmt::Display for PersonId {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "full", derive(DieselNewType, TS))]
#[cfg_attr(feature = "full", ts(export))]
/// The comment id.
pub struct CommentId(pub Uuid);

impl fmt::Display for CommentId {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "full", derive(DieselNewType, TS))]
#[cfg_attr(feature = "full", ts(export))]
/// The community id.
pub struct CommunityId(pub Uuid);

impl fmt::Display for CommunityId {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "full", derive(DieselNewType, TS))]
#[cfg_attr(feature = "full", ts(export))]
/// The local user id.
pub struct LocalUserId(pub Uuid);

impl fmt::Display for LocalUserId {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "full", derive(DieselNewType, TS))]
#[cfg_attr(feature = "full", ts(export))]
/// The private message id.
pub struct PrivateMessageId(pub Uuid);

impl fmt::Display for PrivateMessageId {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "full", derive(DieselNewType, TS))]
#[cfg_attr(feature = "full", ts(export))]
/// The person mention id.
pub struct PersonMentionId(pub Uuid);

impl fmt::Display for PersonMentionId {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "full", derive(DieselNewType, TS))]
#[cfg_attr(feature = "full", ts(export))]
/// The person block id.
pub struct PersonBlockId(Uuid);

impl fmt::Display for PersonBlockId {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "full", derive(DieselNewType, TS))]
#[cfg_attr(feature = "full", ts(export))]
/// The community block id.
pub struct CommunityBlockId(pub Uuid);

impl fmt::Display for CommunityBlockId {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "full", derive(DieselNewType, TS))]
#[cfg_attr(feature = "full", ts(export))]
/// The comment report id.
pub struct CommentReportId(pub Uuid);

impl fmt::Display for CommentReportId {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "full", derive(DieselNewType, TS))]
#[cfg_attr(feature = "full", ts(export))]
/// The post report id.
pub struct PostReportId(pub Uuid);

impl fmt::Display for PostReportId {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "full", derive(DieselNewType, TS))]
#[cfg_attr(feature = "full", ts(export))]
/// The private message report id.
pub struct PrivateMessageReportId(pub Uuid);

impl fmt::Display for PrivateMessageReportId {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "full", derive(DieselNewType, TS))]
#[cfg_attr(feature = "full", ts(export))]
/// The site id.
pub struct SiteId(pub Uuid);
impl fmt::Display for SiteId {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "full", derive(DieselNewType, TS))]
#[cfg_attr(feature = "full", ts(export))]
pub struct LanguageId(pub i32);

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "full", derive(DieselNewType, TS))]
#[cfg_attr(feature = "full", ts(export))]
pub struct LocalUserLanguageId(pub Uuid);
impl fmt::Display for LocalUserLanguageId {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "full", derive(DieselNewType, TS))]
#[cfg_attr(feature = "full", ts(export))]
pub struct SiteLanguageId(pub Uuid);
impl fmt::Display for SiteLanguageId {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "full", derive(DieselNewType, TS))]
#[cfg_attr(feature = "full", ts(export))]
pub struct CommunityLanguageId(pub Uuid);
impl fmt::Display for CommunityLanguageId {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "full", derive(DieselNewType, TS))]
#[cfg_attr(feature = "full", ts(export))]
/// The comment reply id.
pub struct CommentReplyId(pub Uuid);

impl fmt::Display for CommentReplyId {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "full", derive(DieselNewType, TS))]
#[cfg_attr(feature = "full", ts(export))]
/// The instance id.
pub struct InstanceId(pub Uuid);
impl fmt::Display for InstanceId {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "full", derive(DieselNewType, TS))]
#[cfg_attr(feature = "full", ts(export))]
/// The local site id.
pub struct LocalSiteId(pub Uuid);
impl fmt::Display for LocalSiteId {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "full", derive(DieselNewType, TS))]
#[cfg_attr(feature = "full", ts(export))]
/// The custom emoji id.
pub struct CustomEmojiId(pub Uuid);
impl fmt::Display for CustomEmojiId {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "full", derive(DieselNewType, TS))]
#[cfg_attr(feature = "full", ts(export))]
/// The custom emoji id.
pub struct CustomEmojiKeywordId(pub Uuid);
impl fmt::Display for CustomEmojiKeywordId {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "full", derive(DieselNewType, TS))]
#[cfg_attr(feature = "full", ts(export))]
pub struct ActivityId(pub Uuid);

impl fmt::Display for ActivityId {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "full", derive(DieselNewType, TS))]
#[cfg_attr(feature = "full", ts(export))]
pub struct PostLikeId(pub Uuid);

impl fmt::Display for PostLikeId {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "full", derive(DieselNewType, TS))]
#[cfg_attr(feature = "full", ts(export))]
pub struct PostSaveId(pub Uuid);

impl fmt::Display for PostSaveId {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "full", derive(DieselNewType, TS))]
#[cfg_attr(feature = "full", ts(export))]
pub struct PostReadId(pub Uuid);

impl fmt::Display for PostReadId {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "full", derive(DieselNewType, TS))]
#[cfg_attr(feature = "full", ts(export))]
pub struct CommentLikeId(pub Uuid);

impl fmt::Display for CommentLikeId {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "full", derive(DieselNewType, TS))]
#[cfg_attr(feature = "full", ts(export))]
pub struct CommentSavedId(pub Uuid);

impl fmt::Display for CommentSavedId {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "full", derive(DieselNewType, TS))]
#[cfg_attr(feature = "full", ts(export))]
pub struct CommentAggregatesId(pub Uuid);

impl fmt::Display for CommentAggregatesId {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "full", derive(DieselNewType, TS))]
#[cfg_attr(feature = "full", ts(export))]
pub struct CommunityFollowerId(pub Uuid);

impl fmt::Display for CommunityFollowerId {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "full", derive(DieselNewType, TS))]
#[cfg_attr(feature = "full", ts(export))]
pub struct CommunityModeratorId(pub Uuid);

impl fmt::Display for CommunityModeratorId {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "full", derive(DieselNewType, TS))]
#[cfg_attr(feature = "full", ts(export))]
pub struct CommunityPersonBanId(pub Uuid);

impl fmt::Display for CommunityPersonBanId {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "full", derive(DieselNewType, TS))]
#[cfg_attr(feature = "full", ts(export))]
pub struct CommunityAggregatesId(pub Uuid);

impl fmt::Display for CommunityAggregatesId {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "full", derive(DieselNewType, TS))]
#[cfg_attr(feature = "full", ts(export))]
pub struct PersonAggregatesId(pub Uuid);

impl fmt::Display for PersonAggregatesId {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "full", derive(DieselNewType, TS))]
#[cfg_attr(feature = "full", ts(export))]
pub struct PostAggregatesId(pub Uuid);

impl fmt::Display for PostAggregatesId {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "full", derive(DieselNewType, TS))]
#[cfg_attr(feature = "full", ts(export))]
pub struct SiteAggregatesId(pub Uuid);

impl fmt::Display for SiteAggregatesId {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "full", derive(DieselNewType, TS))]
#[cfg_attr(feature = "full", ts(export))]
pub struct PersonPostAggregatesId(pub Uuid);

impl fmt::Display for PersonPostAggregatesId {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "full", derive(DieselNewType, TS))]
#[cfg_attr(feature = "full", ts(export))]
pub struct ModRemovePostId(pub Uuid);

impl fmt::Display for ModRemovePostId {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "full", derive(DieselNewType, TS))]
#[cfg_attr(feature = "full", ts(export))]
pub struct ModLockPostId(pub Uuid);

impl fmt::Display for ModLockPostId {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "full", derive(DieselNewType, TS))]
#[cfg_attr(feature = "full", ts(export))]
pub struct ModFeaturePostId(pub Uuid);

impl fmt::Display for ModFeaturePostId {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "full", derive(DieselNewType, TS))]
#[cfg_attr(feature = "full", ts(export))]
pub struct ModRemoveCommentId(pub Uuid);

impl fmt::Display for ModRemoveCommentId {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "full", derive(DieselNewType, TS))]
#[cfg_attr(feature = "full", ts(export))]
pub struct ModRemoveCommunityId(pub Uuid);

impl fmt::Display for ModRemoveCommunityId {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "full", derive(DieselNewType, TS))]
#[cfg_attr(feature = "full", ts(export))]
pub struct ModBanFromCommunityId(pub Uuid);

impl fmt::Display for ModBanFromCommunityId {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "full", derive(DieselNewType, TS))]
#[cfg_attr(feature = "full", ts(export))]
pub struct ModHideCommunityId(pub Uuid);

impl fmt::Display for ModHideCommunityId {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "full", derive(DieselNewType, TS))]
#[cfg_attr(feature = "full", ts(export))]
pub struct ModAddCommunityId(pub Uuid);

impl fmt::Display for ModAddCommunityId {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "full", derive(DieselNewType, TS))]
#[cfg_attr(feature = "full", ts(export))]
pub struct ModBanId(pub Uuid);

impl fmt::Display for ModBanId {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "full", derive(DieselNewType, TS))]
#[cfg_attr(feature = "full", ts(export))]
pub struct ModAddId(pub Uuid);

impl fmt::Display for ModAddId {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "full", derive(DieselNewType, TS))]
#[cfg_attr(feature = "full", ts(export))]
pub struct ModTransferCommunityId(pub Uuid);

impl fmt::Display for ModTransferCommunityId {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "full", derive(DieselNewType, TS))]
#[cfg_attr(feature = "full", ts(export))]
pub struct AdminPurgePersonId(pub Uuid);

impl fmt::Display for AdminPurgePersonId {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "full", derive(DieselNewType, TS))]
#[cfg_attr(feature = "full", ts(export))]
pub struct AdminPurgeCommunityId(pub Uuid);

impl fmt::Display for AdminPurgeCommunityId {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "full", derive(DieselNewType, TS))]
#[cfg_attr(feature = "full", ts(export))]
pub struct AdminPurgePostId(pub Uuid);

impl fmt::Display for AdminPurgePostId {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "full", derive(DieselNewType, TS))]
#[cfg_attr(feature = "full", ts(export))]
pub struct AdminPurgeCommentId(pub Uuid);

impl fmt::Display for AdminPurgeCommentId {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "full", derive(DieselNewType, TS))]
#[cfg_attr(feature = "full", ts(export))]
pub struct EmailVerificationId(pub Uuid);

impl fmt::Display for EmailVerificationId {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "full", derive(DieselNewType, TS))]
#[cfg_attr(feature = "full", ts(export))]
pub struct PasswordResetId(pub Uuid);

impl fmt::Display for PasswordResetId {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "full", derive(DieselNewType, TS))]
#[cfg_attr(feature = "full", ts(export))]
pub struct RegistrationApplicationId(pub Uuid);

impl fmt::Display for RegistrationApplicationId {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "full", derive(DieselNewType, TS))]
#[cfg_attr(feature = "full", ts(export))]
pub struct PiPaymentId(pub Uuid);

impl fmt::Display for PiPaymentId {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

/*
  PiPaymentIdentifier from Pi Network SDK
*/

//#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[derive(Debug, Clone, Hash, Eq, PartialEq, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "full", derive(DieselNewType, TS))]
#[cfg_attr(feature = "full", ts(export))]
pub struct PiPaymentIdentifier(pub String);

impl fmt::Display for PiPaymentIdentifier {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

/*
  PiUserId from Pi Network SDK
*/
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "full", derive(DieselNewType, TS))]
#[cfg_attr(feature = "full", ts(export))]
pub struct PiUserId(pub Uuid);

impl fmt::Display for PiUserId {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "full", derive(DieselNewType, TS))]
#[cfg_attr(feature = "full", ts(export))]
pub struct LocalSiteRateLimitId(pub Uuid);
impl fmt::Display for LocalSiteRateLimitId {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "full", derive(DieselNewType, TS))]
#[cfg_attr(feature = "full", ts(export))]
pub struct FederationAllowListId(pub Uuid);
impl fmt::Display for FederationAllowListId {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "full", derive(DieselNewType, TS))]
#[cfg_attr(feature = "full", ts(export))]
pub struct FederationBlockListId(pub Uuid);
impl fmt::Display for FederationBlockListId {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "full", derive(DieselNewType, TS))]
#[cfg_attr(feature = "full", ts(export))]
pub struct TaglineId(pub Uuid);
impl fmt::Display for TaglineId {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "full", derive(DieselNewType, TS))]
#[cfg_attr(feature = "full", ts(export))]
pub struct PersonFollowerId(pub Uuid);
impl fmt::Display for PersonFollowerId {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "full", derive(DieselNewType, TS))]
#[cfg_attr(feature = "full", ts(export))]
pub struct PersonBalanceId(pub Uuid);
impl fmt::Display for PersonBalanceId {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "full", derive(DieselNewType, TS))]
#[cfg_attr(feature = "full", ts(export))]
pub struct SecretId(pub Uuid);

impl fmt::Display for SecretId {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

#[cfg(feature = "full")]
#[derive(Serialize, Deserialize)]
#[serde(remote = "Ltree")]
/// Do remote derivation for the Ltree struct
pub struct LtreeDef(pub String);

#[repr(transparent)]
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "full", derive(AsExpression, FromSqlRow))]
#[cfg_attr(feature = "full", diesel(sql_type = diesel::sql_types::Text))]
pub struct DbUrl(pub(crate) Box<Url>);

impl DbUrl {
  pub fn inner(&self) -> &Url {
    &self.0
  }
}

impl Display for DbUrl {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    self.clone().0.fmt(f)
  }
}

// the project doesnt compile with From
#[allow(clippy::from_over_into)]
impl Into<DbUrl> for Url {
  fn into(self) -> DbUrl {
    DbUrl(Box::new(self))
  }
}
#[allow(clippy::from_over_into)]
impl Into<Url> for DbUrl {
  fn into(self) -> Url {
    *self.0
  }
}

#[cfg(feature = "full")]
impl<T> From<DbUrl> for ObjectId<T>
where
  T: Object + Send + 'static,
  for<'de2> <T as Object>::Kind: Deserialize<'de2>,
{
  fn from(value: DbUrl) -> Self {
    let url: Url = value.into();
    ObjectId::from(url)
  }
}

#[cfg(feature = "full")]
impl<T> From<DbUrl> for CollectionId<T>
where
  T: Collection + Send + 'static,
  for<'de2> <T as Collection>::Kind: Deserialize<'de2>,
{
  fn from(value: DbUrl) -> Self {
    let url: Url = value.into();
    CollectionId::from(url)
  }
}

#[cfg(feature = "full")]
impl<T> From<CollectionId<T>> for DbUrl
where
  T: Collection,
  for<'de2> <T as Collection>::Kind: Deserialize<'de2>,
{
  fn from(value: CollectionId<T>) -> Self {
    let url: Url = value.into();
    url.into()
  }
}

impl Deref for DbUrl {
  type Target = Url;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

#[cfg(feature = "full")]
impl TS for DbUrl {
  fn name() -> String {
    "string".to_string()
  }
  fn dependencies() -> Vec<ts_rs::Dependency> {
    Vec::new()
  }
  fn transparent() -> bool {
    true
  }
}
