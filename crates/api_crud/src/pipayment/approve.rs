use crate::pipayment::payment::{PiPaymentInfo, pi_payment_create};
use crate::pipayment::{client::*};
use crate::PerformCrud;
use actix_web::web::Data;
use lemmy_api_common::{context::LemmyContext, pipayment::*, utils::get_local_user_view_from_jwt};
use lemmy_db_schema::source::person::Person;
use lemmy_db_schema::source::pipayment::PiPayment;
use lemmy_utils::{error::LemmyError, ConnectionId};

#[async_trait::async_trait(?Send)]
impl PerformCrud for PiApprove {
  type Response = PiApproveResponse;

  async fn perform(
    &self,
    context: &Data<LemmyContext>,
    _websocket_id: Option<ConnectionId>,
  ) -> Result<PiApproveResponse, LemmyError> {
    let data = self;

    if data.pi_token.is_none() {
      return Err(LemmyError::from_message("Pi access token is missing!"));
    }

    //let local_user_view =
    //  get_local_user_view_from_jwt(&data.auth.unwrap_or_default(), context.pool(), context.secret()).await?;
    //let person_id = local_user_view.person.id;
    //let person = Person::read(context.pool(), person_id).await?;

    let _pi_token = data.pi_token.clone().unwrap();
    let mut _pi_username;
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
        let err_type = format!("Pi Network Server Error: User not found: {}, error: {}", &_pi_token, _e.to_string());
        return Err(LemmyError::from_message(&err_type));
      }
    };

    let _payment = match PiPayment::find_by_pipayment_id(context.pool(), &_payment_id).await
    {
      Ok(c) => {
        return Err(LemmyError::from_message("The payment was approved"));
      }
      Err(_e) => {
        //return Err(LemmyError::from_message("Not approved payment"));
      },
    };

    let mut info = PiPaymentInfo {
      domain: data.domain.clone(),
      pi_token: Some(_pi_token.clone()),
      pi_username: _pi_username.clone(),
      pi_uid: _pi_uid.clone(),
      paymentid: data.paymentid.clone(),
      obj_cat: data.obj_cat.clone(),
      obj_id: data.obj_id,
      ref_id: data.ref_id,
      comment: data.comment.clone(),
      auth: None,
    };

    let paytype ;
    if info.obj_cat.is_some()
    {
      paytype = info.obj_cat.clone().unwrap_or_default();
      if paytype == "reward"
      {
        match Person::find_by_name(context.pool(), &info.comment.clone().unwrap_or_default()).await
        {
          Ok(p) =>{
            if p.external_id.is_none() {
              return Err(LemmyError::from_message("Cannot approve reward the user")); 
            }
            info.ref_id = Some(p.id.0);
          },
          Err(_e) => {
            return Err(LemmyError::from_message("Cannot approve reward non-exist user")); 
          },
        };    
      }
    }
    let _payment = match pi_payment_create(context, &info, None, None).await {
      Ok(c) => c,
      Err(e) => {
        let err_type = e.to_string();
        return Err(LemmyError::from_message(&err_type));
      }
    };
    Ok(PiApproveResponse {
      success: true,
      id: _payment.id,
      paymentid: _payment_id.to_owned(),
    })
  }
}
