use crate::pipayment::client::*;
use crate::PerformCrud;
use actix_web::web::Data;
use lemmy_api_common::{context::LemmyContext, pipayment::*};
use lemmy_db_schema::source::pipayment::PiPayment;
use lemmy_utils::{error::LemmyError, ConnectionId};

#[async_trait::async_trait(?Send)]
impl PerformCrud for CreatePayment {
  type Response = CreatePaymentResponse;

  async fn perform(
    &self,
    context: &Data<LemmyContext>,
    _websocket_id: Option<ConnectionId>,
  ) -> Result<CreatePaymentResponse, LemmyError> {
    let data = self;

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
          "Pi Network Server Error: User not found, error: {}",
          _e.to_string()
        );
        return Err(LemmyError::from_message(&err_type));
      }
    };

    let _payment = match PiPayment::find_by_pipayment_id(context.pool(), &_payment_id).await
    {
      Ok(c) => {
        return Err(LemmyError::from_message("Approve an approved payment"));
        //Some(c)
      }
      Err(_e) => {
        //return Err(LemmyError::from_message("Not approved payment"));
      },
    };

    let _payment = match pi_payment_update(context, &data, None, None).await {
      Ok(c) => c,
      Err(e) => {
        let err_type = e.to_string();
        return Err(LemmyError::from_message(&err_type));
      }
    };
    Ok(CreatePaymentResponse {
      success: true,
      id: _payment.id,
    })
  }
}
