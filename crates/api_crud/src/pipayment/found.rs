use crate::pipayment::payment::pi_payment_update;
use crate::pipayment::{client::*, payment::PiPaymentInfo};
use crate::PerformCrud;
use actix_web::web::Data;
use lemmy_api_common::{pipayment::*, };
use lemmy_db_schema::{
  newtypes::*, source::{pipayment::*, person::*}, traits::Crud,
  utils::naive_now,
};

use lemmy_utils::{error::LemmyError, settings::SETTINGS, ConnectionId};
use lemmy_api_common::{context::LemmyContext};
use uuid::Uuid;

#[async_trait::async_trait(?Send)]
impl PerformCrud for PiPaymentFound {
  type Response = PiPaymentFoundResponse;

  async fn perform(
    &self,
    context: &Data<LemmyContext>,
    _websocket_id: Option<ConnectionId>,
  ) -> Result<PiPaymentFoundResponse, LemmyError> {
    let settings = SETTINGS.to_owned();
    let data: &PiPaymentFound = self;

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
        let err_type = format!(
          "Pi Network Server Error: User not found: {}, error: {}",
          &_pi_token,
          _e.to_string()
        );
        return Err(LemmyError::from_message(&err_type));
      }
    };

    let mut info = PiPaymentInfo {
      domain: data.domain.clone(),
      pi_token: data.pi_token.clone(),
      pi_username: _pi_username.clone(),
      pi_uid: _pi_uid.clone(),
      paymentid: data.paymentid.clone(),
      obj_cat: None,
      obj_id: None,
      ref_id: None,
      comment: Some("PiPaymentFound".to_string()),
      auth: data.auth.clone(),
    };

    let mut finished = false;
    let _payment = match PiPayment::find_by_pipayment_id(context.pool(), &_payment_id).await
    {
      Ok(p) => {
        info.obj_cat = p.obj_cat.clone();
        info.obj_id = p.obj_id.clone();
        info.ref_id = p.ref_id.clone();
        info.comment = p.comment.clone();
        finished = p.finished;
        if p.finished {
          return Err(LemmyError::from_message("The payment found is finished"));
        }
        Some(p)
      }
      Err(_e) => {
        // TODO: Check PaymentDTO, then insert
        println!("PiPaymentFound: NOT FOUND IN LOCAL DATABASE {} {}", _pi_username.clone(), data.paymentid.clone());
        return Err(LemmyError::from_message("Payment not approved "));
      },
    };

    println!("PiPaymentFound update: {} {}, finished: {}", _pi_username.clone(), data.paymentid.clone(), finished);
    let _payment = match pi_payment_update(context, &info, _payment, None).await {
      Ok(c) => c,
      Err(e) => {
        let err_type = e.to_string();
        return Err(LemmyError::from_message(&err_type));
      }
    };

    println!("PiPaymentFoundResponse: {} {}", _pi_username.clone(), data.paymentid.clone());
    let payment = _payment.clone();
    return Ok(PiPaymentFoundResponse {
      id: payment.id,
      paymentid: payment.identifier.unwrap_or_default(),
    });

    
  }
}
