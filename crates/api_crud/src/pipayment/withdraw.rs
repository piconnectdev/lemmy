use crate::pipayment::client::*;
use crate::PerformCrud;
use actix_web::web::Data;
use lemmy_api_common::{context::LemmyContext};
use lemmy_api_common::pipayment::*;

use lemmy_db_schema::source::pipayment::{PiPayment, PiPaymentSafe};
use lemmy_utils::{error::LemmyError, ConnectionId};

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
    let person = Person::read(pool, person_id).await?;
    if !person.verified {
      return Err(LemmyError::from_message("User not verified!"));
    }
    let uuid = Uuid::parse_str(&person.external_id.clone());
    let puid = match uuid {
      Ok(u) => Some(PersonId(u)),
      Err(_e) => {
        return Err(LemmyError::from_message("User not found!"));
      }
    };

    let _pays = match pi_incompleted_server_payments(context.client()).await
    {
      Ok(pays) => {
        if pays.size() > 0 {
          let mut pay_iter = pays.iter();
          for pay in pay_iter {
            if pay.transaction.is_some() {
              println!("Got completed: {}", pay.identifier);
              // match pi_complete(context.client(), pay.identifier).await
              // {
              // }
            } else {
              println!("Got completed: {}", pay.developer_approved);
              // match pi_cancel(context.client(), pay.identifier).await
              // {

              // };
            }
          }
        }
      },
      Err(_e) => {
        return Err(LemmyError::from_message("Server busy!"));
      }
    };
    /// TODO: Check user balances > amount 
    let args = PiPaymentArgs {
      amount: 0.01,
      //amount: data.amount,
      //pub memo: String,
      //pub metadata: Option<Value>,
      uid: person.external_id.clone(),
      memo: data.comment,
    };

    let payment = match pi_create(context.client(), &args).await
    {
      Ok(c) => {
        _payment_id = c.identifier.clone();
        Some(c)
      }
      Err(_e) => {
        return Err(LemmyError::from_message("Not approved payment"));
      },
    };
    // TODO: Submit transaction
    // TODO: Completed transaction
    println!("PiWithdrawResponse: {} {}", person_id.clone(), _paymentid.clone());
    Ok(PiWithdrawResponse {
      id: _payment.id,
      paymentid: "".to_string(),
    })
  }
}
