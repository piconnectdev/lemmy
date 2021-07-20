use crate::pipayment::client::*;
use crate::PerformCrud;
use actix_web::web::Data;
use lemmy_api_common::{blocking, password_length_check, person::*, pipayment::*};
use lemmy_apub::{
  generate_apub_endpoint, generate_followers_url, generate_inbox_url, generate_shared_inbox_url,
  EndpointType,
};
use lemmy_db_queries::{
  source::local_user::LocalUser_, source::site::*, Crud, Followable, Joinable, ListingType,
  SortType,
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
impl PerformCrud for PiApproveRegister {
  type Response = PiResponse;

  async fn perform(
    &self,
    context: &Data<LemmyContext>,
    _websocket_id: Option<ConnectionId>,
  ) -> Result<PiResponse, LemmyError> {
    let data: &PiApproveRegister = self;

    // Make sure site has open registration
    if let Ok(site) = blocking(context.pool(), move |conn| Site::read_simple(conn)).await? {
      if !site.open_registration {
        return Err(ApiError::err("registration_closed").into());
      }
    }

    password_length_check(&data.password)?;

    // Check if there are admins. False if admins exist
    let no_admins = blocking(context.pool(), move |conn| {
      PersonViewSafe::admins(conn).map(|a| a.is_empty())
    })
    .await??;

    // If its not the admin, check the captcha
    if !no_admins && Settings::get().captcha().enabled {
      let check = context
        .chat_server()
        .send(CheckCaptcha {
          uuid: data
            .captcha_uuid
            .to_owned()
            .unwrap_or_else(|| "".to_string()),
          answer: data
            .captcha_answer
            .to_owned()
            .unwrap_or_else(|| "".to_string()),
        })
        .await?;
      if !check {
        return Err(ApiError::err("captcha_incorrect").into());
      }
    }

    check_slurs(&data.username)?;

    if !is_valid_username(&data.username) {
      return Err(ApiError::err("invalid_username").into());
    }
    //check_slurs_opt(&data.paymentid.unwrap())?;
    //check_slurs_opt(&data.username)?;
    let _payment_id = data.paymentid.to_owned();
    let _user = data.username.to_owned();
    let _payment_url = data.paymentid.clone();

    let found_payment = match blocking(context.pool(), move |conn| {
      Payment::find(_payment_url.to_owned().to_string())
    })
    .await?
    .await?
    {
      Ok(c) => c,
      Err(_e) => None,
    };
    let _payment_dto: PiPaymentDto = pi_approve(context.client(), &_payment_url.clone()).await?;
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
      paymentid: data.paymentid.to_owned().to_string(),
      username: data.username.to_owned(),
    })
  }
}

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
    let _payment_id = data.paymentid.to_owned();
    let _user = data.username.to_owned();
    let _payment_url = data.paymentid.clone();

    //let found_payment = blocking(context.pool(), move |conn| Payment::find(_payment_url)).await??;
    let _payment_dto: PiPaymentDto = pi_approve(context.client(), &_payment_url).await?;
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
      paymentid: data.paymentid.to_owned().to_string(),
      username: data.username.to_owned(),
    })
  }
}
