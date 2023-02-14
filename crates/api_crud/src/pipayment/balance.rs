use crate::PerformCrud;
use actix_web::web::Data;
use lemmy_api_common::pipayment::*;
use lemmy_db_views_actor::community_moderator_view::*;
use lemmy_db_views_actor::person_view::*;
use lemmy_utils::{error::LemmyError, ConnectionId};
use lemmy_api_common::{context::LemmyContext};

#[async_trait::async_trait(?Send)]
impl PerformCrud for GetPiBalances {
  type Response = GetPiBalancesResponse;

  async fn perform(
    &self,
    context: &Data<LemmyContext>,
    _websocket_id: Option<ConnectionId>,
  ) -> Result<GetPiBalancesResponse, LemmyError> {
    let data: &GetPiBalances = self;

    // let sort = data.sort;
    // let page = data.page;
    // let limit = data.limit;
    // let unread_only = data.unread_only;
    // let mut payments = PiPaymentQuery::builder()
    //   .pool(context.pool())
    //   .person_id(person_id)
    //   .page(page)
    //   .limit(limit)
    //   .out(false)
    //   .build()
    //   .list()
    //   .await
    //   .map_err(|e| LemmyError::from_error_message(e, "couldnt_get_payment"))?;

    let res = GetPiBalancesResponse {
      id: None,
      status: Some("".to_string()), 
      asset: Some("PI".to_string()),
      deposited: 0.0,
      rewarded: 0.0,
      amount: 0.0,
      pending: 0.0,
      withdrawed: 0.0,
    };
    Ok(res)
  }
}
