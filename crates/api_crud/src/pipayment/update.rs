use crate::pipayment::client::*;
use crate::PerformCrud;
use actix_web::web::Data;
use lemmy_api_common::pipayment::*;
//use lemmy_api_common::{
//  blocking, get_local_user_view_from_jwt, is_admin,
//  site::{EditSite, SiteResponse},
//  site_description_length_check,
//};
// use lemmy_db_queries::{
//   diesel_option_overwrite, diesel_option_overwrite_to_url, source::site::Site_, Crud,
// };
//use lemmy_db_schema::naive_now;
//use lemmy_db_views::site_view::SiteView;

use lemmy_utils::{ ApiError, ConnectionId, LemmyError};
use lemmy_websocket::{messages::SendAllMessage, LemmyContext, UserOperationCrud};

#[async_trait::async_trait(?Send)]
impl PerformCrud for PiTip {
  type Response = PiTipResponse;
  async fn perform(
    &self,
    context: &Data<LemmyContext>,
    websocket_id: Option<ConnectionId>,
  ) -> Result<PiTipResponse, LemmyError> {
    let data: &PiTip = self;
    //let payment_url = &Url{data.paymentid.to_owned()};
    let _payment_id = data.paymentid.to_owned();
    let _pi_username = data.pi_username.to_owned();
    let _pi_uid = data.pi_uid.clone();
    let _tx = Some(data.txid.clone());
    let approve = PiApprove{
        paymentid: data.paymentid.clone(),
        pi_username: data.pi_username.clone(),
        pi_uid:  data.pi_uid.clone(),
        person_id: data.person_id.clone(),
        comment: data.comment.clone(),
        auth: data.auth.clone(),
    };

    //let local_user_view = get_local_user_view_from_jwt(&data.auth, context.pool()).await?;

    //check_slurs_opt(&data.paymentid)?;
    //check_slurs_opt(&data.username)?;
    /*
    let found_payment = blocking(context.pool(), move |conn| {
      Payment::find(data.paymentid.as_ref())
    })
    .await??;
    let payment_url = payment_url;
    */

    // let payment_dto = pi_complete(
    //   context.client(),
    //   &payment_url.to_owned(),
    //   &data.txid.to_owned(),
    // )
    // .await?;
    //let payment_id = payment_dto;

    /*
    let payment_form = PaymentForm {
      payment_id: found_payment.paymentid.as_ref(),
      username: found_payment.username.as_ref(),
      //updated: Some(naive_now()),
    };

    let update_payment = move |conn: &'_ _| Payment::update(conn, found_payment.id, &payment_form);
    if blocking(context.pool(), update_payment).await?.is_err() {
      return Err(ApiError::err("couldnt_update_payment").into());
    }
    */
    //let site_view = blocking(context.pool(), move |conn| SiteView::read(conn)).await??;
    let _payment =
      match pi_update_payment(context, &approve, _tx).await {
        Ok(c) => c,
        Err(e) => {
          let err_type = e.to_string();
          return Err(ApiError::err(&err_type).into());
        }
      };
    Ok(PiTipResponse {
      id: _payment.id,
      paymentid: _payment_id.to_owned(),
    })
  }
}
