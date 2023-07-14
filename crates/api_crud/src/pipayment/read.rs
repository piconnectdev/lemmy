use crate::PerformCrud;
use actix_web::web::Data;
use lemmy_api_common::context::LemmyContext;
use lemmy_api_common::pipayment::*;
use lemmy_utils::{error::LemmyError, };
#[async_trait::async_trait(?Send)]
impl PerformCrud for GetPayment {
  type Response = GetPaymentResponse;

  async fn perform(&self, context: &Data<LemmyContext>) -> Result<GetPaymentResponse, LemmyError> {
    let data: &GetPayment = self;

    let pmid = data.id.to_owned();
    let res = GetPaymentResponse { payment: None };
    Ok(res)
  }
}
