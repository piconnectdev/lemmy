use crate::PerformCrud;
use actix_web::web::Data;
use lemmy_api_common::pipayment::*;
use lemmy_db_views_actor::community_moderator_view::*;
use lemmy_db_views_actor::person_view::*;
use lemmy_utils::{error::LemmyError, ConnectionId};
use lemmy_api_common::{context::LemmyContext};

#[async_trait::async_trait(?Send)]
impl PerformCrud for GetPayments {
  type Response = GetPaymentsResponse;

  async fn perform(
    &self,
    context: &Data<LemmyContext>,
    _websocket_id: Option<ConnectionId>,
  ) -> Result<GetPaymentsResponse, LemmyError> {
    let data: &GetPayments = self;

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

    // let pmid = data.id.to_owned();
    // let res = GetPiPaymentResponse {
    //   pid: "".to_string(), 
    // };
    // Ok(res)
    return Err(LemmyError::from_message("Not yet implements"));
  }
}
