use crate::PerformCrud;
use actix_web::web::Data;
use lemmy_api_common::{context::LemmyContext};
use lemmy_api_common::{
  person::LoginResponse,
  pipayment::*,
};
use lemmy_db_schema::source::local_site::RegistrationMode;
use lemmy_db_views::structs::{SiteView};

use lemmy_utils::{
  error::LemmyError,
  ConnectionId,
};

use crate::web3::ext::*;
use super::client::{pi_me};

#[async_trait::async_trait(?Send)]
impl PerformCrud for PiRegister {
  type Response = LoginResponse;

  async fn perform(
    &self,
    context: &Data<LemmyContext>,
    _websocket_id: Option<ConnectionId>,
  ) -> Result<LoginResponse, LemmyError> {
    let data: &PiRegister = &self;
    let mut ext_account = data.ea.clone();

    let site_view = SiteView::read_local(context.pool()).await?;
    let local_site = site_view.local_site;

    if local_site.registration_mode == RegistrationMode::Closed {
      return Err(LemmyError::from_message("registration_closed"));
    }

    if local_site.site_setup {
      if !context.settings().pi_enabled {
        return Err(LemmyError::from_message("registration_disabled"));
      }
      if !context.settings().pinetwork.pi_allow_all {
        return Err(LemmyError::from_message("registration_disabled"));
      }
    }
    let _pi_token = ext_account.token.clone();
    let mut _pi_username = ext_account.account.clone();
    let mut _pi_uid = None;

    // First, valid user token
    let user_dto = match pi_me(context, &_pi_token.clone()).await {
      Ok(dto) => {
        _pi_username = dto.username.clone();
        _pi_uid = Some(dto.uid.clone());
        ext_account.extra = Some(dto.uid.clone().0.hyphenated().to_string());
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

    //ext_account.extra = _pi_uid.to_string();
    let login_response = match create_external_account(context, &_pi_username.clone(), &ext_account, &data.info, false).await
    {
      Ok(c) => c,
      Err(_e) => {
        return Err(LemmyError::from_message("registration_disabled"));
      },
    };
    
    Ok(login_response)
  }
}
