use actix_web::web::Data;
use lemmy_api_common::context::LemmyContext;
use lemmy_utils::error::LemmyError;

mod comment;
mod community;
mod custom_emoji;
pub mod pipayment;
mod post;
mod private_message;
mod site;
mod user;
mod web3;

#[async_trait::async_trait(?Send)]
pub trait PerformCrud {
  type Response: serde::ser::Serialize + Send;

  async fn perform(&self, context: &Data<LemmyContext>) -> Result<Self::Response, LemmyError>;
}
