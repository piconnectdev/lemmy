use crate::PerformCrud;
use actix_web::web::Data;
use lemmy_api_common::pipayment::*;
use lemmy_db_views_actor::community_moderator_view::*;
use lemmy_db_views_actor::person_view::*;
use lemmy_utils::{error::LemmyError, ConnectionId};
use lemmy_api_common::{context::LemmyContext};

#[async_trait::async_trait(?Send)]
impl PerformCrud for PiKey {
  type Response = PiKeyResponse;

  async fn perform(
    &self,
    context: &Data<LemmyContext>,
    _websocket_id: Option<ConnectionId>,
  ) -> Result<PiKeyResponse, LemmyError> {
    let data: &PiKey = self;

    let res = PiKeyResponse {
      success: true,
      id: None,
    };
    Ok(res)
  }
}
