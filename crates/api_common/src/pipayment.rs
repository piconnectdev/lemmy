use lemmy_db_views::{
  comment_view::CommentView, post_view::PostView, private_message_view::PrivateMessageView,
};
use lemmy_db_views_actor::{
  community_follower_view::CommunityFollowerView, community_moderator_view::CommunityModeratorView,
  person_mention_view::PersonMentionView, person_view::PersonViewSafe,
};
use serde::{Deserialize, Serialize};


#[derive(Deserialize)]
pub struct PiApprove {
  pub paymentid: String,
  pub username: String,
}

#[derive(Deserialize)]
pub struct PiResponse {
  pub paymentid: String,
  pub username: String,
}

#[derive(Deserialize)]
pub struct PiTip {
  pub username: String,
  pub paymentid: String,
  pub auth: String,
}

#[derive(Deserialize)]
pub struct PiTipResponse {
  pub paymentid: String,
}

