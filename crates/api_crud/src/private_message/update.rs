use crate::PerformCrud;
use actix_web::web::Data;
use lemmy_api_common::{
  context::LemmyContext,
  private_message::{EditPrivateMessage, PrivateMessageResponse},
  utils::{get_local_user_view_from_jwt, local_site_to_slur_regex},
  websocket::{send::send_pm_ws_message, UserOperationCrud},
};
use lemmy_db_schema::{
  source::{
    local_site::LocalSite,
    private_message::{PrivateMessage, PrivateMessageUpdateForm},
  },
  traits::{Crud, Signable}, 
  utils::naive_now,
};
use lemmy_utils::{error::LemmyError, utils::remove_slurs, ConnectionId};

#[async_trait::async_trait(?Send)]
impl PerformCrud for EditPrivateMessage {
  type Response = PrivateMessageResponse;

  #[tracing::instrument(skip(self, context, websocket_id))]
  async fn perform(
    &self,
    context: &Data<LemmyContext>,
    websocket_id: Option<ConnectionId>,
  ) -> Result<PrivateMessageResponse, LemmyError> {
    let data: &EditPrivateMessage = self;
    let local_user_view =
      get_local_user_view_from_jwt(&data.auth, context.pool(), context.secret()).await?;
    let local_site = LocalSite::read(context.pool()).await?;

    // Checking permissions
    let private_message_id = data.private_message_id;
    let orig_private_message = PrivateMessage::read(context.pool(), private_message_id).await?;
    if local_user_view.person.id != orig_private_message.creator_id {
      return Err(LemmyError::from_message("no_private_message_edit_allowed"));
    }

    // Doing the update
    let content_slurs_removed = remove_slurs(&data.content, &local_site_to_slur_regex(&local_site));
    let private_message_id = data.private_message_id;
    let updated_private_message = PrivateMessage::update(
      context.pool(),
      private_message_id,
      &PrivateMessageUpdateForm::builder()
        .content(Some(content_slurs_removed))
        .updated(Some(Some(naive_now())))
        .build(),
    )
    .await
    .map_err(|e| LemmyError::from_error_message(e, "couldnt_update_private_message"))?;

    let (signature, _meta, _content)  = PrivateMessage::sign_data(&updated_private_message.clone()).await;
    let updated_private_message = PrivateMessage::update_srv_sign(context.pool(), updated_private_message.id.clone(), signature.clone().unwrap_or_default().as_str())
      .await
      .map_err(|e| LemmyError::from_error_message(e, "couldnt_update_private_message"))?;

    let op = UserOperationCrud::EditPrivateMessage;
    send_pm_ws_message(data.private_message_id, op, websocket_id, context).await
  }
}
