use crate::pipayment::client::*;
use crate::PerformCrud;
use actix_web::web::Data;
use lemmy_api_common::context::LemmyContext;
use lemmy_api_common::pipayment::*;
use lemmy_api_common::utils::local_user_view_from_jwt;

use lemmy_db_schema::newtypes::{PersonId, PiUserId};
use lemmy_db_schema::source::person::Person;
use lemmy_db_schema::source::person_balance::PersonBalance;
use lemmy_db_schema::source::pipayment::{PiPayment, PiPaymentInsertForm};
use lemmy_db_schema::traits::Crud;
use lemmy_utils::{error::LemmyError, };
use uuid::Uuid;

#[async_trait::async_trait(?Send)]
impl PerformCrud for PiWithdraw {
  type Response = PiWithdrawResponse;
  async fn perform(&self, context: &Data<LemmyContext>) -> Result<PiWithdrawResponse, LemmyError> {
    let data: &PiWithdraw = self;
    let local_user_view = local_user_view_from_jwt(&data.auth, context).await?;
    let person_id = local_user_view.person.id;
    let person = Person::read(context.pool(), person_id.clone()).await?;
    let uuid = Uuid::parse_str(&person.external_id.clone().unwrap());
    let mut pi_uid = match uuid {
      Ok(u) => PiUserId(u),
      Err(_e) => {
        return Err(LemmyError::from_message("User's external_id os not valid!"));
      }
    };
    if !person.verified {
      return Err(LemmyError::from_message("You're not verified!"));
    };

    // Must use Pi Browser for withdraw?

    let pi_token = data.pi_token.clone().unwrap();
    let pi_username;

    // First, valid user token
    match pi_me(context, &pi_token.clone()).await {
      Ok(dto) => {
        pi_username = dto.username.clone();
        if pi_username != person.external_name.clone().unwrap_or_default() {
          let err_str = format!(
            "Error: Not same pi user: {} {} local:{} me:{}",
            pi_username, &pi_token, pi_uid, dto.uid
          );
          return Err(LemmyError::from_message(&err_str));
        }
        pi_uid = dto.uid;
        Some(dto)
      }
      Err(_e) => {
        let err_type = format!(
          "Pi Network Server Error: User not found: {}, error: {}",
          &pi_token,
          _e.to_string()
        );
        return Err(LemmyError::from_message(&err_type));
      }
    };

    /*
    match PiPayment::find_withdraw_pending(context.pool(), &person_id.clone()).await
    {
      Ok(pays) => {
        if pays.len() > 0 {
          return Err(LemmyError::from_message("Withdraw is in pending queue!"));
        }
      },
      Err(_e) => {
      }
    };
    */

    let mut _payment_id: String;
    let fee = 0.01;
    let amount = f64::trunc(data.amount * 10000000.0) / 10000000.0;

    if amount <= 0.0 || amount > 10000.0 {
      return Err(LemmyError::from_message(
        "Invalid withdraw balance (0.0 < amount < 10000.0)!",
      ));
    }
    match PersonBalance::find_by_asset(context.pool(), person_id.clone(), "PI").await {
      Ok(balance) => {
        if balance.amount < (fee + amount) {
          return Err(LemmyError::from_message("Balance not enough!"));
        }
      }
      Err(_e) => {
        return Err(LemmyError::from_message("Balance record not found!"));
      }
    };
    match PersonBalance::update_withdraw(context.pool(), person_id.clone(), amount, fee).await {
      Ok(balance) => {}
      Err(_e) => {
        return Err(LemmyError::from_message("Update PI balance error!"));
      }
    };
    let memo = format!("{} withraw: {} ", person.name.clone(), amount.clone());
    let payment_form = PiPaymentInsertForm::builder()
      .domain(data.domain.clone())
      .instance_id(Some(person.instance_id))
      .person_id(Some(person_id.clone()))
      .obj_cat(Some("withdraw".to_string()))
      .obj_id(None)
      .a2u(1)
      .asset(data.asset.clone())
      .fee(fee)
      .ref_id(Some(person_id.clone().0))
      .comment(data.comment.clone())
      .testnet(context.settings().pinetwork.pi_testnet)
      .finished(false)
      .updated(None)
      .pi_uid(Some(pi_uid))
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
      .metadata(None)
      .extras(None)
      .build();

    let payment = match PiPayment::create(context.pool(), &payment_form).await {
      Ok(payment) => payment,
      Err(_e) => {
        let err_str = _e.to_string();
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
