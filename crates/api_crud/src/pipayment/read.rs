use crate::pipayment::client::*;
use crate::PerformCrud;
use actix_web::web::Data;
use lemmy_api_common::{blocking, get_local_user_view_from_jwt_opt, mark_post_as_read, post::*};
use lemmy_api_common::{person::*, pipayment::*};
use lemmy_apub::{
  generate_local_apub_endpoint, generate_followers_url, generate_inbox_url, generate_shared_inbox_url,
  EndpointType,
};
use lemmy_db_views_actor::person_view::PersonViewSafe;
use lemmy_db_views_actor::{
  community_moderator_view::CommunityModeratorView, community_view::CommunityView,
};
use lemmy_utils::{
  apub::generate_actor_keypair,
  claims::Claims,
  request::*,
  settings::structs::Settings,
  utils::{check_slurs, is_valid_actor_name},
  ApiError, ConnectionId, LemmyError,
};
use lemmy_websocket::{messages::CheckCaptcha, LemmyContext};

#[async_trait::async_trait(?Send)]
impl PerformCrud for GetPayment {
  type Response = GetPaymentResponse;

  async fn perform(
    &self,
    context: &Data<LemmyContext>,
    _websocket_id: Option<ConnectionId>,
  ) -> Result<GetPaymentResponse, LemmyError> {
    let data: &GetPayment = self;

    let pmid = data.id.to_owned();
    let res = GetPaymentResponse {
      pid: "".to_string(),
    };
    Ok(res)
  }
}
