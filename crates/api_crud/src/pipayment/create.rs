use crate::PerformCrud;
use actix_web::web::Data;
use lemmy_api_common::pipayment::PiApprove;
use lemmy_api_common::{blocking, password_length_check, person::*, pipayment::*};
use lemmy_apub::{
  generate_apub_endpoint, generate_followers_url, generate_inbox_url, generate_shared_inbox_url,
  EndpointType,
};
use lemmy_db_queries::{
  source::{local_user::LocalUser_, site::Site_},
  Crud, Followable, Joinable, ListingType, SortType,
};
use lemmy_db_schema::{
  source::{
    community::*,
    local_user::{LocalUser, LocalUserForm},
    person::*,
    site::*,
  },
  CommunityId,
};
use lemmy_db_views_actor::person_view::PersonViewSafe;
use lemmy_utils::{
  apub::generate_actor_keypair,
  claims::Claims,
  pipayment::PiPaymentDto,
  request::*,
  utils::{check_slurs, is_valid_username},
  ApiError, ConnectionId, LemmyError,
};
use lemmy_websocket::{messages::CheckCaptcha, LemmyContext};
use uuid::Uuid;

#[async_trait::async_trait(?Send)]
impl PerformCrud for PiApprove {
  type Response = PiResponse;

  async fn perform(
    &self,
    context: &Data<LemmyContext>,
    _websocket_id: Option<ConnectionId>,
  ) -> Result<PiResponse, LemmyError> {
    let data: &PiApprove = self;

    //check_slurs_opt(&data.paymentid.unwrap())?;
    //check_slurs_opt(&data.username)?;
    let paymentId = data.paymentid.to_owned();
    let user = data.username.to_owned();
    let payment_url = data.paymentid.to_owned();
    /*
    let found_payment = blocking(context.pool(), move |conn| {
      Payment::find(data.paymentid.as_ref())
    })
    .await??;
    */
    let paymentDto: PiPaymentDto = pi_approve(context.client(), &payment_url).await?;
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
      created_at: paymentDto.created_at,
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
    // Return the jwt
    Ok(PiResponse {
      paymentid: data.paymentid.to_owned(),
      username: data.username.to_owned(),
    })
  }
}
