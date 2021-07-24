use crate::pipayment::client::*;
use crate::PerformCrud;
use actix_web::web::Data;
use lemmy_api_common::{blocking, password_length_check, person::*, pipayment::*};
use lemmy_apub::{
  generate_apub_endpoint, generate_followers_url, generate_inbox_url, generate_shared_inbox_url,
  EndpointType,
};
use lemmy_db_queries::{
  source::local_user::LocalUser_, source::site::*, source::pipayment::*, Crud, Followable, Joinable, ListingType,
  SortType,
};
use lemmy_db_schema::source::{
  community::*,
  local_user::{LocalUser, LocalUserForm},
  person::*,
  pipayment::*,
  site::*,
};
use lemmy_db_views_actor::person_view::PersonViewSafe;
use lemmy_utils::{
  apub::generate_actor_keypair,
  claims::Claims,
  request::*,
  settings::structs::Settings,
  utils::{check_slurs, check_slurs_opt, is_valid_username},
  ApiError, ConnectionId, LemmyError,
};
use lemmy_websocket::{messages::CheckCaptcha, LemmyContext};
use uuid::Uuid;

#[async_trait::async_trait(?Send)]
impl PerformCrud for PiAgreeRegister {
  type Response = PiAgreeResponse;

  async fn perform(
    &self,
    context: &Data<LemmyContext>,
    _websocket_id: Option<ConnectionId>,
  ) -> Result<PiAgreeResponse, LemmyError> {
    let data: &PiAgreeRegister = self;

    // Make sure site has open registration
    if let Ok(site) = blocking(context.pool(), move |conn| Site::read_simple(conn)).await? {
      if !site.open_registration {
        return Err(ApiError::err("registration_closed").into());
      }
    }

    password_length_check(&data.info.password)?;

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
          uuid: data.info
            .captcha_uuid
            .to_owned()
            .unwrap_or_else(|| "".to_string()),
          answer: data.info
            .captcha_answer
            .to_owned()
            .unwrap_or_else(|| "".to_string()),
        })
        .await?;
      if !check {
        return Err(ApiError::err("captcha_incorrect").into());
      }
    }

    check_slurs(&data.info.username)?;

    if !is_valid_username(&data.info.username) {
      return Err(ApiError::err("invalid_username").into());
    }
    //check_slurs_opt(&data.paymentid.unwrap())?;
    //check_slurs_opt(&data.info.username)?;

    let _payment_id = data.paymentid.to_owned();
    let _pi_username = data.pi_username.to_owned();
    let _pi_uid = data.pi_uid.clone();

    let mut approved = false;
    let mut completed = false;
    let mut exist = false;
    //let mut fetch_pi_server = true;
    let mut pid;
    let mut _payment = match blocking(context.pool(), move |conn| {
      PiPayment::find_by_pipayment_id(&conn, _payment_id)
    })
    .await?
    {
      Ok(c) => {
        exist = true;
        approved = c.approved;
        completed = c.completed;
        pid = c.id;
        Some(c)
      },
      Err(_e) => {
        None
      },
    };

    let mut dto: Option<PiPaymentDto> = None;

    if (_payment.is_some()) {
      if (!approved) {
        let dto = match pi_approve(context.client(), &_payment_id.clone())
        .await
        {
          Ok(c) => Some(c),
          Err(_e) => None,
        };
      }
    } else {
      dto = match pi_approve(context.client(), &_payment_id.clone())
      .await
      {
        Ok(c) => Some(c),
        Err(_e) => None,
      };
    }

    if !exist || ! approved {
      
    } else {

    }


    // if _payment.is_some() {

    // }
    //let mut pm = None;

    let mut _payment_dto = PiPaymentDto{
      ..PiPaymentDto::default()
    };

    if dto.is_some() {
      _payment_dto = dto.unwrap();
    }

    let refid = match &data.info.captcha_uuid {
      Some(uid) =>  match Uuid::parse_str(uid) {
        Ok(uidx) => Some(uidx),
        Err(_e) => None,
      }
      None => None,
    };

    let mut payment_form = PiPaymentForm {
      person_id: None,
      ref_id: refid,
      testnet: Settings::get().pi_testnet(),
      finished: false,
      updated: None,
      pi_payment_id: data.paymentid,
      pi_uid: data.pi_uid,
      pi_username: data.pi_username.clone(),
      comment: data.comment.clone(),

      identifier: _payment_dto.identifier,
      user_uid: _payment_dto.user_uid,
      amount: _payment_dto.amount,
      memo: _payment_dto.memo,
      to_address: _payment_dto.to_address,
      created_at: _payment_dto.created_at,
      approved: _payment_dto.status.developer_approved,
      verified: _payment_dto.status.transaction_verified,
      completed: _payment_dto.status.developer_completed,
      cancelled: _payment_dto.status.cancelled,
      user_cancelled: _payment_dto.status.user_cancelled,
      tx_link: "".to_string(),
      tx_id: "".to_string(),
      tx_verified: false,
      //tx_id:  _payment_dto.transaction.map(|tx| tx.txid),
      //payment_dto: _payment_dto,
      //..PiPaymentForm::default()
      metadata: None,
      //dto: None,

    };

    match _payment_dto.transaction {
      Some(tx) =>  {
        payment_form.tx_link = tx._link; 
        payment_form.tx_verified = tx.verified;
        payment_form.tx_id = tx.txid;          
      },
      None => {}
  }

    if !exist {
    _payment = match blocking(context.pool(), move |conn| {
      PiPayment::create(&conn, &payment_form)
    })
    .await?
    {
      Ok(payment) => Some(payment),
      Err(e) => {
        // let err_type = if e.to_string() == "value too long for type character varying(200)" {
        //   "post_title_too_long"
        // } else {
        //   "couldnt_create_post"
        // };
        let err_type = e.to_string();
        return Err(ApiError::err(&err_type).into());
      }
    };
    pid = _payment.unwrap().id;

  } else {
    let pmt = _payment.unwrap();
    pid = pmt.id;
    let inserted_payment = match blocking(context.pool(), move |conn| {
      PiPayment::update(&conn, pid, &payment_form)
    })
    .await?
    {
      Ok(payment) => payment,
      Err(e) => {
        // let err_type = if e.to_string() == "value too long for type character varying(200)" {
        //   "post_title_too_long"
        // } else {
        //   "couldnt_create_post"
        // };
        let err_type = e.to_string();
        return Err(ApiError::err(&err_type).into());
      }
    };
  }
    
    //_payment = pi_update_payment(context, payment_id, &_pi_username, _pi_uid, Some(data.info)).await?;
    // Return the jwt
    Ok(PiAgreeResponse {
      id: pid,
      //id: _payment.map(|x| x.id),
      paymentid: _payment_id.to_owned(),
    })
  }
}

