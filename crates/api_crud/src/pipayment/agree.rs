use crate::pipayment::payment::pi_payment_create;
use crate::pipayment::{client::*, payment::PiPaymentInfo};
use crate::PerformCrud;
use actix_web::web::Data;
use lemmy_api_common::context::LemmyContext;
use lemmy_api_common::{pipayment::*, utils::password_length_check};
use lemmy_db_schema::{
  source::{person::*, pipayment::*},
  traits::Crud,
  RegistrationMode,
};
use lemmy_db_views::structs::SiteView;
use lemmy_utils::{error::LemmyError, };
use uuid::Uuid;

#[async_trait::async_trait(?Send)]
impl PerformCrud for PiAgreeRegister {
  type Response = PiAgreeResponse;

  async fn perform(&self, context: &Data<LemmyContext>) -> Result<PiAgreeResponse, LemmyError> {
    let data: &PiAgreeRegister = self;

    let site_view = SiteView::read_local(context.pool()).await?;
    let local_site = site_view.local_site;

    if local_site.registration_mode == RegistrationMode::Closed {
      return Err(LemmyError::from_message("registration_closed"));
    } else {
      return Err(LemmyError::from_message("registration_disabled"));
    }

    if local_site.site_setup {
      if !context.settings().pi_enabled {
        return Err(LemmyError::from_message("registration_disabled"));
      }
      if !context.settings().pinetwork.pi_allow_all {
        return Err(LemmyError::from_message("registration_disabled"));
      }
    }

    let mut result_string = "".to_string();
    let mut result = true;
    let mut completed = false;

    password_length_check(&data.info.password)?;

    let _pi_token = data.ea.token.clone();
    let mut _pi_username = data.ea.account.clone();
    let mut _pi_uid = None;
    let _payment_id = data.paymentid.clone();

    let _new_user = data.info.username.clone();

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
          &data.ea.account,
          _e.to_string()
        );
        return Err(LemmyError::from_message(&err_type));
      }
    };

    let mut approved = false;
    let mut completed = false;
    let mut exist = false;
    //let mut fetch_pi_server = true;
    let mut pid;
    let mut dto: Option<PiPaymentDto> = None;

    let mut _payment = match PiPayment::find_by_pipayment_id(context.pool(), &_payment_id).await {
      Ok(c) => {
        exist = true;
        approved = c.approved;
        completed = c.completed;
        pid = c.id;
        Some(c)
      }
      Err(_e) => None,
    };

    if _payment.is_some() {
      if approved || completed {
        return Ok(PiAgreeResponse {
          success: true,
          id: None,
          paymentid: data.paymentid.to_owned(),
          extra: None,
        });
      }
    }

    let other_person = match Person::find_by_extra_name(context.pool(), &_pi_username.clone()).await
    {
      Ok(c) => Some(c),
      Err(_e) => None,
    };

    let person = match Person::find_by_name(context.pool(), &_new_user).await {
      Ok(c) => Some(c),
      Err(_e) => None,
    };

    match other_person {
      Some(pi) => {
        match person {
          Some(per) => {
            if pi.external_name != per.external_name {
              let err_type = format!(
                "User {} is exist and belong to other Pi Network account",
                &data.info.username
              );
              result_string = err_type.clone();
              result = false
            } else {
              // Same name and account: change password ???
              result = true;
            }
          }
          None => {
            // Not allow change username ???
            let err_type = format!("Your account already exist: {}", pi.name);
            result_string = err_type.clone();
            result = false;
          }
        };
      }
      None => {
        match person {
          Some(per) => {
            let err_type = format!(
              "User {} is exist, create same user name is not allow!",
              &data.info.username
            );
            result_string = err_type.clone();
            result = false;
          }
          None => {
            // No account, we approved this tx
            result = true;
          }
        };
      }
    }

    // TODO: UUID check
    let refid = match &data.info.captcha_uuid {
      Some(uid) => match Uuid::parse_str(uid) {
        Ok(uidx) => Some(uidx),
        Err(_e) => None,
      },
      None => None,
    };

    let info = PiPaymentInfo {
      domain: data.domain.clone(),
      pi_token: Some(_pi_token.clone()),
      pi_username: _pi_username.clone(),
      pi_uid: _pi_uid.clone(),
      paymentid: data.paymentid.clone(),
      obj_cat: Some("register".to_string()),
      obj_id: None,
      ref_id: None,
      comment: Some("register".to_string()),
      auth: None,
    };

    let _payment = match pi_payment_create(context, &info, None, None).await {
      Ok(c) => c,
      Err(e) => {
        let err_type = e.to_string();
        return Err(LemmyError::from_message(&err_type));
      }
    };

    Ok(PiAgreeResponse {
      success: result,
      id: Some(_payment.id),
      paymentid: data.paymentid.to_owned(),
      extra: Some(result_string),
    })
  }
}
