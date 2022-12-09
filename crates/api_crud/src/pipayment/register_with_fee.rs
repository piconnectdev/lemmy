use crate::PerformCrud;
use actix_web::web::Data;
use lemmy_api_common::person::LoginResponse;
use lemmy_api_common::{context::LemmyContext};
use lemmy_api_common::{
  pipayment::*,
};
use lemmy_db_schema::{
  source::{*
  },
};
use lemmy_db_views::structs::SiteView;
use lemmy_utils::{
  error::LemmyError,
  settings::SETTINGS,
  ConnectionId,
};
use crate::web3::ext::*;

use super::client::{pi_payment_update, pi_me};

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

    if !local_site.open_registration {
      return Err(LemmyError::from_message("registration_closed"));
    }
    if local_site.site_setup {
      if !context.settings().pi_enabled {
        println!("PiRegisterWithFee: not pi_enabled: {} ", data.paymentid.clone());
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

    let approve = PiApprove {
      domain: data.domain.clone(),
      pi_token: Some(_pi_token.clone()),
      pi_username: _pi_username.clone(),
      pi_uid: _pi_uid,
      paymentid: _payment_id.clone(),
      
      person_id: None,
      comment: None,
      auth: None,
    };

    println!("call pi_payment_update: {} - {} ", _pi_username.clone(), _payment_id.clone());
    let payment = match pi_payment_update(context, &approve.clone(), Some(data.txid.clone())).await
    {
      Ok(p) => {
        if !p.completed {
          println!("PiRegisterWithFee: not completed: {} ", p.identifier.clone());
          //return Err(LemmyError::from_message("registration_disabled"));
        }
        Some(p)
      },
      Err(_c) => {
        println!("PiRegisterWithFee: pi_payment_update: {} ", _c.to_string());
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
    Ok(login_response)
  }
}
