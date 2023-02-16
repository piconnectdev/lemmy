use crate::PerformCrud;
use crate::pipayment::payment::PiPaymentInfo;
use actix_web::web::Data;
use lemmy_api_common::person::LoginResponse;
use lemmy_api_common::{context::LemmyContext};
use lemmy_api_common::{
  pipayment::*,
};
use lemmy_db_schema::source::local_site::RegistrationMode;
use lemmy_db_schema::source::pipayment::PiPayment;
use lemmy_db_views::structs::SiteView;
use lemmy_utils::{
  error::LemmyError,
  ConnectionId,
};
use crate::web3::ext::*;

use super::client::{pi_me};
use super::payment::{pi_payment_update};

#[async_trait::async_trait(?Send)]
impl PerformCrud for PiRegisterWithFee {
  type Response = LoginResponse;
  async fn perform(
    &self,
    context: &Data<LemmyContext>,
    _websocket_id: Option<ConnectionId>,
  ) -> Result<LoginResponse, LemmyError> {
    let data: &PiRegisterWithFee = &self;
    let ext_account = data.ea.clone();

    let site_view = SiteView::read_local(context.pool()).await?;
    let local_site = site_view.local_site;

    if local_site.registration_mode == RegistrationMode::Closed {
      return Err(LemmyError::from_message("registration_closed"));
    } else {
      return Err(LemmyError::from_message("registration_disabled"));
    }
    if local_site.site_setup {
      if !context.settings().pi_enabled {
        //println!("PiRegisterWithFee: not pi_enabled: {} ", data.paymentid.clone());
        return Err(LemmyError::from_message("registration_disabled"));
      }
      // if !context.settings().pinetwork.pi_allow_all {
      //   return Err(LemmyError::from_message("registration_disabled"));
      // }
    }

    let _pi_token = ext_account.token.clone();
    let mut _pi_username = ext_account.account.clone();
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
          &_pi_username.clone(),
          _e.to_string()
        );
        return Err(LemmyError::from_message(&err_type));
      }
    };

    let info = PiPaymentInfo {
      domain: data.domain.clone(),
      pi_token: Some(_pi_token.clone()),
      pi_username: _pi_username.clone(),
      pi_uid: _pi_uid,
      paymentid: _payment_id.clone(),
      obj_cat: None,
      obj_id: None,
      ref_id: None,
      comment: Some("register_with_fee".to_string()),
      auth: None,
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

    let payment = match pi_payment_update(context, &info.clone(), _payment, Some(data.txid.clone())).await
    {
      Ok(p) => {
        if !p.completed {
          //println!("PiRegisterWithFee: not completed: {} ", p.identifier.clone());
          return Err(LemmyError::from_message("registration_disabled"));
        }
        Some(p)
      },
      Err(_c) => {
        //println!("PiRegisterWithFee: pi_payment_update: {} ", _c.to_string());
        return Err(LemmyError::from_message("registration_disabled"));
      },
    };

    let login_response = match create_external_account(context, &_pi_username.clone(), &ext_account.clone(), &data.info.clone(), true).await
    {
      Ok(c) => c,
      Err(_e) => {
        println!("PiRegisterWithFee: create_external_account error: {} {} {}", &_pi_username.clone(), &data.info.username.clone(), data.paymentid.clone());
        return Err(LemmyError::from_message("registration_disabled"));
        //None
      },
    };
    println!("PiRegisterWithFee: {} {}", _pi_username.clone(), data.paymentid.clone());
    Ok(login_response)
  }
}
