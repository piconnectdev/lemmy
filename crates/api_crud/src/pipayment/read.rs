use crate::PerformCrud;
use actix_web::web::Data;
use lemmy_api_common::pipayment::*;
use lemmy_utils::{error::LemmyError, ConnectionId};
use lemmy_api_common::{context::LemmyContext};
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
      payment: None,
    };
    Ok(res)
  }
}
