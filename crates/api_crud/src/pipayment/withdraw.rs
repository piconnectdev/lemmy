use crate::pipayment::client::*;
use crate::PerformCrud;
use actix_web::web::Data;
use lemmy_api_common::utils::get_local_user_view_from_jwt;
use lemmy_api_common::{context::LemmyContext};
use lemmy_api_common::pipayment::*;

use lemmy_db_schema::newtypes::{PersonId, PiUserId};
use lemmy_db_schema::source::person::Person;
use lemmy_db_schema::source::pipayment::{PiPayment, PiPaymentSafe, PiPaymentInsertForm};
use lemmy_utils::{error::LemmyError, ConnectionId};
use lemmy_db_schema::traits::Crud;
use uuid::Uuid;

#[async_trait::async_trait(?Send)]
impl PerformCrud for PiWithdraw {
  type Response = PiWithdrawResponse;
  async fn perform(
    &self,
    context: &Data<LemmyContext>,
    websocket_id: Option<ConnectionId>,
  ) -> Result<PiWithdrawResponse, LemmyError> {
    let data: &PiWithdraw = self;
    let local_user_view =
      get_local_user_view_from_jwt(&data.auth, context.pool(), context.secret()).await?;
    let person_id = local_user_view.person.id;
    let mut _payment_id: String;
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
      .obj_cat(Some("withdraw".to_string()))
      .obj_id(None)
      .a2u(true)
      .asset(data.asset.clone())
      .ref_id(None)
      .comment(None)
      .testnet(context.settings().pinetwork.pi_testnet)
      
      .finished( false)
      .updated( None)
      .pi_uid(puid)
      .pi_username( person.external_name.clone().unwrap_or_default() )
      
      .identifier( "".to_string())
      .user_uid( person.external_id.clone().unwrap_or_default())
      .amount( data.amount)
      .memo( "".to_string())
      .from_address( "".to_string())
      .to_address( "".to_string())
      .direction( "".to_string())
      .network( "".to_string())
      .created_at( None)
      .approved( false)
      .verified( false)
      .completed( false)
      .cancelled( false)
      .user_cancelled( false)
      .tx_link("".to_string())
      .tx_id( "".to_string())
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
      

    Ok(PiWithdrawResponse {
      status: Some("PENDING".to_string()),
      id: Some(payment.id),
      pipayid: None,
    })
  }
}
