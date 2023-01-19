use crate::pipayment::client::*;
use crate::PerformCrud;
use actix_web::web::Data;
use lemmy_api_common::{pipayment::*, };
use lemmy_db_schema::{
  newtypes::{PiPaymentId, PersonId}, source::{pipayment::*, person::*}, traits::Crud,
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
    let mut _pi_username = data.pi_username.to_owned();
    let mut _pi_uid = data.pi_uid.clone();

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
          &data.pi_username,
          _e.to_string()
        );
        return Err(LemmyError::from_message(&err_type));
      }
    };

    //let _tx = Some(data.txid.clone());
    let approve = PiApprove {
      domain: data.domain.clone(),
      pi_token: data.pi_token.clone(),
      pi_username: _pi_username.clone(),
      pi_uid: _pi_uid.clone(),
      paymentid: data.paymentid.clone(),
      object_id: None,
      comment: Some("PiPaymentFound".to_string()),
      auth: data.auth.clone(),
    };

    let _payment = match PiPayment::find_by_pipayment_id(context.pool(), &_payment_id).await
    {
      Ok(c) => {
        Some(c)
      }
      Err(_e) => {
        return Err(LemmyError::from_message("Not approved payment"));
      },
    };

    let _payment = match pi_payment_update(context, &approve, _payment, None).await {
      Ok(c) => c,
      Err(e) => {
        let err_type = e.to_string();
        return Err(LemmyError::from_message(&err_type));
      }
    };

    println!("PiPaymentFoundResponse, update: {} {}", _pi_username.clone(), data.paymentid.clone());
    let payment = _payment.clone();
    return Ok(PiPaymentFoundResponse {
      id: payment.id,
      paymentid: payment.identifier,
    });

    
  }
}
