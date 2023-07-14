use crate::PerformCrud;
use actix_web::web::Data;
use lemmy_api_common::{
  context::LemmyContext,
  pipayment::*,
  utils::{is_admin, local_user_view_from_jwt},
};
use lemmy_db_schema::{
  newtypes::{PersonId, PiUserId},
  source::{
    person::Person,
    pipayment::{PiPayment, PiPaymentInsertForm},
  },
  traits::Crud,
};
use lemmy_utils::{error::LemmyError, };
use uuid::Uuid;

#[async_trait::async_trait(?Send)]
impl PerformCrud for CreatePayment {
  type Response = CreatePaymentResponse;

  async fn perform(
    &self,
    context: &Data<LemmyContext>,
  ) -> Result<CreatePaymentResponse, LemmyError> {
    let data = self;

    let local_user_view = local_user_view_from_jwt(&data.auth, context).await?;

    is_admin(&local_user_view)?;

    let person_id = local_user_view.person.id.clone();
    let person = Person::read(context.pool(), person_id).await?;
    if !person.verified {
      return Err(LemmyError::from_message("User is not verified!"));
    }

    let uuid = Uuid::parse_str(&person.external_id.clone().unwrap());
    let puid = match uuid {
      Ok(u) => Some(PiUserId(u)),
      Err(_e) => {
        return Err(LemmyError::from_message("User's external_id not valid!"));
      }
    };

    let fee = 0.01;
    let amount = f64::trunc(data.amount * 10000000.0) / 10000000.0;

    if amount <= 0.0 || amount > 10000.0 {
      return Err(LemmyError::from_message(
        "Invalid withdraw balance (0.0 < amount < 10000.0)!",
      ));
    }

    let memo = format!("Send {}: {} ", person.name.clone(), amount.clone());
    let payment_form = PiPaymentInsertForm::builder()
      .domain(data.domain.clone())
      .instance_id(Some(person.instance_id))
      .person_id(Some(person_id.clone()))
      .obj_cat(data.obj_cat.clone())
      .obj_id(data.obj_id.clone())
      .a2u(1)
      .asset(data.asset.clone())
      .fee(fee)
      .ref_id(data.ref_id.clone())
      .comment(None)
      .testnet(context.settings().pinetwork.pi_testnet)
      .finished(true) // TODO: This feature for test only
      .updated(None)
      .pi_uid(puid)
      .pi_username(person.external_name.clone().unwrap_or_default())
      .identifier(None)
      .user_uid(person.external_id.clone())
      .amount(amount)
      .memo(Some(memo))
      .from_address(None)
      .to_address(None)
      .direction(None)
      .network(None)
      .created_at(None)
      .approved(false)
      .verified(false)
      .completed(false)
      .cancelled(false)
      .user_cancelled(false)
      .tx_link(None)
      .tx_id(None)
      .tx_verified(false)
      .metadata(None) //_payment_dto.metadata,
      .extras(None)
      .build();
    let payment = match PiPayment::create(context.pool(), &payment_form).await {
      Ok(payment) => payment,
      Err(_e) => {
        let err_str = _e.to_string();
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
