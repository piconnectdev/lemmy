use crate::PerformCrud;
use actix_web::web::Data;
use lemmy_api_common::context::LemmyContext;
use lemmy_api_common::pipayment::*;
use lemmy_api_common::pipayment::*;
use lemmy_api_common::utils::{is_admin, local_user_view_from_jwt};
use lemmy_db_schema::source::pipayment::*;
use lemmy_db_schema::source::{person::*, pipayment::*};
use lemmy_db_schema::traits::Crud;
use lemmy_db_views_actor::community_moderator_view::*;
use lemmy_utils::{error::LemmyError, };

#[async_trait::async_trait(?Send)]
impl PerformCrud for GetPayments {
  type Response = GetPaymentsResponse;

  async fn perform(&self, context: &Data<LemmyContext>) -> Result<GetPaymentsResponse, LemmyError> {
    let data: &GetPayments = self;

    let local_user_view = local_user_view_from_jwt(&data.auth, context).await?;
    let mut person_id = local_user_view.person.id;
    //let person = Person::read(context.pool(), person_id).await?;
    //let mut payments: Option<Vec<PiPayment>> = None;
    match is_admin(&local_user_view) {
      Ok(x) => {
        if data.person_id.is_some() {
          person_id = data.person_id.unwrap_or_default();
        }
      }
      Err(e) => {}
    };
    let payments = match PiPayment::find_by_person(context.pool(), &person_id.clone()).await {
      Ok(pays) => Some(pays),
      Err(_e) => {
        return Err(LemmyError::from_message("Payments not found"));
      }
    };
    // let sort = data.sort;
    // let page = data.page;
    // let limit = data.limit;
    // let unread_only = data.unread_only;
    // let mut payments = PiPaymentQuery::builder()
    //   .pool(context.pool())
    //   .person_id(person_id)
    //   .page(page)
    //   .limit(limit)
    //   .out(false)
    //   .build()
    //   .list()
    //   .await
    //   .map_err(|e| LemmyError::from_error_message(e, "couldnt_get_payment"))?;

    // let pmid = data.id.to_owned();
    // let res = GetPiPaymentResponse {
    //   pid: "".to_string(),
    // };
    // Ok(res)
    Ok(GetPaymentsResponse {
      pipayments: payments,
    })
  }
}
