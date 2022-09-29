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

use lemmy_utils::{error::LemmyError, settings::SETTINGS, ConnectionId};
use lemmy_websocket::LemmyContext;

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
    let approve = PiApprove {
      paymentid: data.paymentid.clone(),
      pi_username: data.pi_username.clone(),
      pi_uid: data.pi_uid.clone(),
      person_id: data.person_id.clone(),
      comment: data.comment.clone(),
      auth: data.auth.clone(),
    };

    let _payment = match pi_update_payment(context, &approve, _tx).await {
      Ok(c) => c,
      Err(e) => {
        let err_type = e.to_string();
        return Err(LemmyError::from_message(&err_type));
      }
    };
    Ok(PiTipResponse {
      id: _payment.id,
      paymentid: _payment_id.to_owned(),
    })
  }
}
