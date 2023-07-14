use crate::PerformCrud;
use actix_web::web::Data;
use lemmy_api_common::{context::LemmyContext, person::LoginResponse, web3::*};
use lemmy_db_schema::{
  RegistrationMode,
  source::local_site::*, //newtypes::PersonId,
                                        //source::{
                                        //local_user::{LocalUser, LocalUserInsertForm},
                                        //person::*, registration_application::RegistrationApplicationInsertForm,
                                        //},
                                        //traits::{Crud}, aggregates::structs::PersonAggregates,
  *,
};
use lemmy_db_views::structs::SiteView;

use lemmy_utils::{
  claims::Claims, error::LemmyError, settings::SETTINGS, utils::web3::eth_verify, ConnectionId,
};

use crate::web3::ext::*;

#[async_trait::async_trait(?Send)]
impl PerformCrud for Web3Register {
  type Response = LoginResponse;
  async fn perform(&self, context: &Data<LemmyContext>) -> Result<LoginResponse, LemmyError> {
    let settings = SETTINGS.to_owned();
    let data: &Web3Register = &self;
    let mut ext_account = data.ea.clone();

    // no email verification, or applications if the site is not setup yet
    let (mut email_verification, mut require_application) = (false, false);

    let mut _result = true;

    let site_view = SiteView::read_local(context.pool()).await?;
    let local_site = site_view.local_site;

    let require_registration_application =
      local_site.registration_mode == RegistrationMode::RequireApplication;
    if local_site.registration_mode == RegistrationMode::Closed {
      return Err(LemmyError::from_message("registration_closed"));
    }

    if local_site.site_setup {
      if !settings.web3_enabled {
        return Err(LemmyError::from_message("registration_disabled"));
      }
    }

    // If its not the admin, check the token / sign
    if local_site.site_setup {
      let mut _address = ext_account.account.clone();
      let mut _signature = ext_account.signature.clone().unwrap();
      let _token = ext_account.token.clone();
      let _cli_time = ext_account.epoch;

      let check = context
        .chat_server()
        .check_token(_token.clone(), "".to_string())?;
      if !check {
        return Err(LemmyError::from_message("token_incorrect"));
      }

      let text = format!(
        "LOGIN:{};TOKEN:{};TIME:{}",
        _address.clone(),
        _token.clone(),
        _cli_time.clone()
      );
      println!(
        "Web3Registration is processing for {} - {} {} {} ",
        text.clone(),
        _address.clone(),
        _token.clone(),
        _signature.clone()
      );
      if !eth_verify(_address.clone(), text.clone(), _signature) {
        return Err(LemmyError::from_message("captcha_incorrect"));
      }
    }

    ext_account.extra = Some(ext_account.account.clone());
    let login_response = match create_external_account(
      context,
      &ext_account.account.clone(),
      &ext_account,
      &data.info.clone(),
      false,
    )
    .await
    {
      Ok(c) => c,
      Err(_e) => {
        return Err(_e);
      }
    };

    Ok(login_response)
  }
}
