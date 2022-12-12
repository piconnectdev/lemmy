use crate::PerformCrud;
use actix_web::web::Data;
use lemmy_api_common::pipayment::*;
use lemmy_utils::{error::LemmyError, ConnectionId};
use lemmy_api_common::{context::LemmyContext};
#[async_trait::async_trait(?Send)]
impl PerformCrud for GetPiPayment {
  type Response = GetPiPaymentResponse;

  async fn perform(
    &self,
    context: &Data<LemmyContext>,
    _websocket_id: Option<ConnectionId>,
  ) -> Result<GetPiPaymentResponse, LemmyError> {
    let data: &GetPiPayment = self;

    let pmid = data.id.to_owned();
    let res = GetPiPaymentResponse {
      pid: "".to_string(),
    };
    Ok(res)
  }
}
