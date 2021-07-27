use crate::pipayment::client::*;
use crate::PerformCrud;
use actix_web::web::Data;
use lemmy_api_common::{blocking, password_length_check, person::*, pipayment::*};
use lemmy_apub::{
  generate_apub_endpoint, generate_followers_url, generate_inbox_url, generate_shared_inbox_url,
  EndpointType,
};
use lemmy_db_queries::{
  source::local_user::LocalUser_, source::pipayment::PiPayment_, source::site::*, Crud, Followable,
  Joinable, ListingType, SortType,
};
use lemmy_db_schema::source::{
  community::*,
  local_user::{LocalUser, LocalUserForm},
  person::*,
  pipayment::*,
  site::*,
};
//use lemmy_db_views_actor::person_view::PersonViewSafe;
use lemmy_utils::{
  apub::generate_actor_keypair,
  claims::Claims,
  request::*,
  settings::structs::Settings,
  utils::{check_slurs, is_valid_username},
  ApiError, ConnectionId, LemmyError,
};
use lemmy_websocket::{messages::CheckCaptcha, LemmyContext};
use uuid::Uuid;

#[async_trait::async_trait(?Send)]
impl PerformCrud for PiApprove {
  type Response = PiApproveResponse;

  async fn perform(
    &self,
    context: &Data<LemmyContext>,
    _websocket_id: Option<ConnectionId>,
  ) -> Result<PiApproveResponse, LemmyError> {
    let data: &PiApprove = self;

    //check_slurs_opt(&data.paymentid.unwrap())?;
    //check_slurs_opt(&data.username)?;
    let _payment_id = data.paymentid.clone();
    let _pi_username = data.pi_username.to_owned();
    let _pi_uid = data.pi_uid.clone();

    /*

    let _payment = match blocking(context.pool(), move |conn| {
      PiPayment::find_by_pipayment_id(&conn, _payment_id)
    })
    .await?
    {
       Ok(c) => c,
       Err(_e) => {},
    };
    if (_payment.is_ok()) {
      let x = _payment.unwrap();
    }

    let _payment_dto: PiPaymentDto = pi_approve(context.client(), &_payment_url).await?;
    */
    // Make sure site has open registration
    /*
    let payment_form = PaymentForm {
      payment_id: paymentId,
      person_name: user,
      identifier: paymentDto.identifier,
      user_uid: paymentDto.user_uid,
      amount: paymentDto.amount,
      memo: paymentDto.memo,
      to_address: paymentDto.to_address,
      created_at: Some(chrono::NaiveDateTime::parse_from_str(&_payment_dto.created_at).unwrap(), "%Y-%m-%dT%H:%M:%S%.f%z")),
      developer_approved: paymentDto.status.developer_approved,
      transaction_verified: paymentDto.status.transaction_verified,
      developer_completed: paymentDto.status.developer_completed,
      cancelled: paymentDto.status.cancelled,
      user_cancelled: paymentDto.status.user_cancelled,
      //tx_id =  ,
      //tx_verified: bool,
      //tx_link: String,
      //payment_dto: ,
      ..PaymentForm::default()
    };

    let inserted_payment = match blocking(context.pool(), move |conn| {
      Payment::create(conn, &payment_form)
    })
    .await?
    {
      Ok(payment) => payment,
      Err(e) => {
        let err_type = if e.to_string() == "value too long for type character varying(200)" {
          "post_title_too_long"
        } else {
          "couldnt_create_post"
        };

        return Err(ApiError::err(err_type).into());
      }
    };

    let inserted_payment_id = inserted_payment.id;
    */
    let _payment =
      match pi_update_payment(context, &_payment_id, &_pi_username, _pi_uid, None).await {
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
