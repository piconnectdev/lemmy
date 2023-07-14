use crate::PerformCrud;
use actix_web::web::Data;
use lemmy_api_common::context::LemmyContext;
use lemmy_api_common::pipayment::*;
use lemmy_api_common::utils::local_user_view_from_jwt;
use lemmy_db_schema::source::person_balance::PersonBalance;
use lemmy_utils::{error::LemmyError, };

#[async_trait::async_trait(?Send)]
impl PerformCrud for GetPiBalances {
  type Response = GetPiBalancesResponse;

  async fn perform(
    &self,
    context: &Data<LemmyContext>,
  ) -> Result<GetPiBalancesResponse, LemmyError> {
    let data: &GetPiBalances = self;

    let local_user_view = local_user_view_from_jwt(&data.auth, context).await?;
    let person_id = local_user_view.person.id;

    let balance = PersonBalance::find_by_asset(context.pool(), person_id, "PI").await?;
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

    let res = GetPiBalancesResponse {
      id: Some(balance.id),
      status: Some("".to_string()),
      asset: balance.asset,
      spent: balance.spent,
      deposited: balance.deposited,
      received: balance.received,
      withdrawed: balance.withdrawed,
      amount: balance.amount,
      pending: balance.pending,
    };

    Ok(res)
  }
}
