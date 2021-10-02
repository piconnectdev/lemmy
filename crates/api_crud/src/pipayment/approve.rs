use crate::pipayment::client::*;
use crate::PerformCrud;
use actix_web::web::Data;
use lemmy_api_common::{pipayment::*};
use lemmy_utils::{
  ApiError, ConnectionId, LemmyError,
};
use lemmy_websocket::{LemmyContext};

#[async_trait::async_trait(?Send)]
impl PerformCrud for PiApprove {
  type Response = PiApproveResponse;

  async fn perform(
    &self,
    context: &Data<LemmyContext>,
    _websocket_id: Option<ConnectionId>,
  ) -> Result<PiApproveResponse, LemmyError> {
    let data = &self;

    let _payment_id = data.paymentid.clone();
    let _pi_username = data.pi_username.to_owned();
    let _pi_uid = data.pi_uid.clone();


    let _payment =
      match pi_update_payment(context, &data, None).await {
        Ok(c) => c,
        Err(e) => {
          let err_type = e.to_string();
          return Err(ApiError::err(&err_type).into());
        }
      };
    Ok(PiApproveResponse {
      id: _payment.id,
      paymentid: _payment_id.to_owned(),
    })
  }
}
