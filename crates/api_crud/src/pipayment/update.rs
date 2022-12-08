use crate::pipayment::client::*;
use crate::PerformCrud;
use actix_web::web::Data;
use lemmy_api_common::{context::LemmyContext};
use lemmy_api_common::pipayment::*;

use lemmy_utils::{error::LemmyError, ConnectionId};

#[async_trait::async_trait(?Send)]
impl PerformCrud for PiTip {
  type Response = PiTipResponse;
  async fn perform(
    &self,
    context: &Data<LemmyContext>,
    websocket_id: Option<ConnectionId>,
  ) -> Result<PiTipResponse, LemmyError> {
    let data: &PiTip = self;
    //let payment_url = &Url{data.paymentid.to_owned()};
    let _payment_id = data.paymentid.to_owned();
    let _pi_username = data.pi_username.to_owned();
    let _pi_uid = data.pi_uid.clone();
    let _tx = Some(data.txid.clone());
    let approve = PiApprove {
      domain: data.domain.clone(),
      pi_token: data.pi_token.clone(),
      paymentid: data.paymentid.clone(),
      pi_username: data.pi_username.clone(),
      pi_uid: data.pi_uid.clone(),
      person_id: data.person_id.clone(),
      comment: data.comment.clone(),
      auth: data.auth.clone(),
    };

    let _payment = match pi_payment_update(context, &approve, _tx).await {
      Ok(c) => c,
      Err(e) => {
        let err_type = e.to_string();
        return Err(LemmyError::from_message(&err_type));
      }
    };
    Ok(PiTipResponse {
      id: _payment.id,
      paymentid: _payment_id.to_owned(),
    })
  }
}

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

