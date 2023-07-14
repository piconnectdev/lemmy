use crate::pipayment::client::*;
use crate::pipayment::payment::{pi_payment_update, PiPaymentInfo};
use crate::PerformCrud;
use actix_web::web::Data;
use lemmy_api_common::context::LemmyContext;
use lemmy_api_common::pipayment::*;

use lemmy_db_schema::source::pipayment::PiPayment;
use lemmy_utils::{error::LemmyError, };

#[async_trait::async_trait(?Send)]
impl PerformCrud for PiPaymentComplete {
  type Response = PiPaymentCompleteResponse;
  async fn perform(
    &self,
    context: &Data<LemmyContext>,
  ) -> Result<PiPaymentCompleteResponse, LemmyError> {
    let data: &PiPaymentComplete = self;
    if data.pi_token.is_none() {
      return Err(LemmyError::from_message("Pi token is missing!"));
    }

    let _pi_token = data.pi_token.clone().unwrap();
    let mut _pi_username;
    let mut _pi_uid = None;

    let _payment_id = data.paymentid.clone();

    // First, valid user token
    let user_dto = match pi_me(context, &_pi_token.clone()).await {
      Ok(dto) => {
        _pi_username = dto.username.clone();
        _pi_uid = Some(dto.uid.clone());
        Some(dto)
      }
      Err(_e) => {
        // Pi Server error
        let err_str = format!(
          "Pi Network Server Error: User not found: {}, error: {}",
          &_pi_token,
          _e.to_string()
        );
        return Err(LemmyError::from_message(&err_str));
      }
    };

    let _tx = Some(data.txid.clone());
    let mut info = PiPaymentInfo {
      domain: data.domain.clone(),
      pi_token: data.pi_token.clone(),
      pi_username: _pi_username.clone(),
      pi_uid: _pi_uid.clone(),
      paymentid: data.paymentid.clone(),
      obj_cat: None,
      obj_id: None,
      ref_id: None,
      comment: data.comment.clone(),
      auth: data.auth.clone(),
    };

    let mut finished = false;
    let _payment = match PiPayment::find_by_pipayment_id(context.pool(), &_payment_id).await {
      Ok(p) => {
        info.obj_cat = p.obj_cat.clone();
        info.obj_id = p.obj_id.clone();
        info.ref_id = p.ref_id.clone();
        info.comment = p.comment.clone();
        finished = p.finished;
        if p.finished {
          return Err(LemmyError::from_message("The payment update is finished"));
        }
        Some(p)
      }
      Err(_e) => {
        return Err(LemmyError::from_message("Completed a payment not approved"));
      }
    };
    let _payment = match pi_payment_update(context, &info, _payment, _tx).await {
      Ok(c) => c,
      Err(e) => {
        let err_str = e.to_string();
        return Err(LemmyError::from_message(&err_str));
      }
    };
    Ok(PiPaymentCompleteResponse {
      id: _payment.id,
      paymentid: _payment_id.to_owned(),
    })
  }
}

#[async_trait::async_trait(?Send)]
impl PerformCrud for PiKey {
  type Response = PiKeyResponse;

  async fn perform(&self, context: &Data<LemmyContext>) -> Result<PiKeyResponse, LemmyError> {
    let data: &PiKey = self;

    let res = PiKeyResponse {
      success: true,
      id: None,
    };
    Ok(res)
  }
}
