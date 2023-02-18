use crate::pipayment::client::*;
use crate::PerformCrud;
use actix_web::web::Data;
use lemmy_api_common::{context::LemmyContext, pipayment::*, utils::{get_local_user_view_from_jwt, is_admin}};
use lemmy_db_schema::{source::{pipayment::{PiPayment, PiPaymentInsertForm}, person::Person}, traits::Crud, newtypes::{PersonId, PiUserId}};
use lemmy_utils::{error::LemmyError, ConnectionId};
use uuid::Uuid;

#[async_trait::async_trait(?Send)]
impl PerformCrud for CreatePayment {
  type Response = CreatePaymentResponse;

  async fn perform(
    &self,
    context: &Data<LemmyContext>,
    _websocket_id: Option<ConnectionId>,
  ) -> Result<CreatePaymentResponse, LemmyError> {
    let data = self;

    let local_user_view =
      get_local_user_view_from_jwt(&data.auth, context.pool(), context.secret()).await?;

    is_admin(&local_user_view)?;

    let person_id = local_user_view.person.id.clone();
    let person = Person::read(context.pool(), person_id).await?;
    if !person.verified {
      return Err(LemmyError::from_message("User not verified!"));
    }
    
    let uuid = Uuid::parse_str(&person.external_id.clone().unwrap());
    let puid = match uuid {
      Ok(u) => Some(PiUserId(u)),
      Err(_e) => {
        return Err(LemmyError::from_message("User not found!"));
      }
    };

    let mut payment_form = PiPaymentInsertForm::builder()
      .domain(data.domain.clone())
      //.instance_id(None)
      .person_id( Some(person_id.clone()))
      .obj_cat(data.obj_cat.clone())
      .obj_id(data.obj_id.clone())
      .a2u(true)
      .asset(data.asset.clone())
      .ref_id(data.ref_id.clone())
      .comment(None)
      .testnet(context.settings().pinetwork.pi_testnet)
      
      .finished(false)
      .updated(None)
      .pi_uid(puid)
      .pi_username(person.external_name.clone().unwrap_or_default() )
      
      .identifier(None)
      .user_uid(person.external_id.clone())
      .amount(data.amount.unwrap_or_default())
      .memo(None)
      .from_address(None)
      .to_address(None)
      .direction(None)
      .network(None)
      .created_at( None)
      .approved(false)
      .verified(false)
      .completed(false)
      .cancelled(false)
      .user_cancelled( false)
      .tx_link(None)
      .tx_id(None)
      .tx_verified( false)
      .metadata( None) //_payment_dto.metadata,
      .extras( None)
      .build();
      let payment = match PiPayment::create(context.pool(), &payment_form).await
      {
        Ok(payment) => {
          println!("CreatePayment, create payment success: {}", payment.id.clone());
          payment
        }
        Err(_e) => {
          let err_str = _e.to_string();
          println!("CreatePayment, create payment error: {}", err_str.clone());
          return Err(LemmyError::from_message(&err_str));
        }
      };

    Ok(CreatePaymentResponse {
      success: true,
      id: payment.id,
      pipayid: None,
    })
  }
}
