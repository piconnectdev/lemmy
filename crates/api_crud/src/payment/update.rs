use crate::PerformCrud;
use actix_web::web::Data;
use lemmy_api_common::{
  blocking, get_local_user_view_from_jwt, is_admin,
  site::{EditSite, SiteResponse},
  site_description_length_check,
};
use lemmy_db_queries::{
  diesel_option_overwrite, diesel_option_overwrite_to_url, source::site::Site_, Crud,
};
use lemmy_db_schema::{
  naive_now,
  source::site::{Site, SiteForm},
};
use lemmy_db_views::site_view::SiteView;
use lemmy_utils::{utils::check_slurs_opt, ApiError, ConnectionId, LemmyError};
use lemmy_websocket::{messages::SendAllMessage, LemmyContext, UserOperationCrud};

#[async_trait::async_trait(?Send)]
impl PerformCrud for PiTip {
  type Response = PiResponse;
  async fn perform(
    &self,
    context: &Data<LemmyContext>,
    websocket_id: Option<ConnectionId>,
  ) -> Result<PiResponse, LemmyError> {
    let data: &PiTip = self;
    let local_user_view = get_local_user_view_from_jwt(&data.auth, context.pool()).await?;

    check_slurs_opt(&data.paymentid)?;
    check_slurs_opt(&data.username)?;

    let found_payment = blocking(context.pool(), move |conn| {
      Payment::find(data.paymentid.as_ref())
    })
    .await??;

    let payment_url = found_payment.paymentid.as_ref();
    let paymentDto = pi_complete(context.client(), data_url).await;
    let paymentId = paymentDto;

    let payment_form = PaymentForm {
      payment_id: found_payment.paymentid.as_ref(),
      username: found_payment.username.as_ref(),
      //updated: Some(naive_now()),
    };

    let update_payment = move |conn: &'_ _| Payment::update(conn, found_payment.id, &payment_form);
    if blocking(context.pool(), update_payment).await?.is_err() {
      return Err(ApiError::err("couldnt_update_payment").into());
    }

    //let site_view = blocking(context.pool(), move |conn| SiteView::read(conn)).await??;

    let res = PiResponse {
      paymentid: found_payment.paymentid.as_ref(),
      username: found_payment.username.as_ref(),
    };

    // context.chat_server().do_send(SendAllMessage {
    //   op: UserOperationCrud::EditSite,
    //   response: res.clone(),
    //   websocket_id,
    // });

    Ok(res)
  }
}
