use crate::PerformCrud;
use actix_web::web::Data;
use lemmy_api_common::{
  build_response::build_community_response,
  community::{CommunityResponse, DeleteCommunity},
  context::LemmyContext,
  utils::{is_top_mod, local_user_view_from_jwt},
};
use lemmy_db_schema::{
  source::{community::{Community, CommunityUpdateForm}, person::Person},
  traits::{Crud, ApubActor},
};
use lemmy_db_views_actor::structs::CommunityModeratorView;
use lemmy_utils::error::LemmyError;

#[async_trait::async_trait(?Send)]
impl PerformCrud for DeleteCommunity {
  type Response = CommunityResponse;

  #[tracing::instrument(skip(context))]
  async fn perform(&self, context: &Data<LemmyContext>) -> Result<CommunityResponse, LemmyError> {
    let data: &DeleteCommunity = self;
    let local_user_view = local_user_view_from_jwt(&data.auth, context).await?;

    // Fetch the community mods
    let community_id = data.community_id;
    let community = Community::read(context.pool(), community_id).await?;
    if community.is_home {
      return Err(LemmyError::from_message("only_admins_can_delete_home"));
    }
    // match Person::read_from_name(context.pool(), &community.name.clone(), true).await
    // {
    //   Ok(p) => {
    //     return Err(LemmyError::from_message(
    //       "only_admins_can_delete_home",
    //     ));
    //   },
    //   Err(e) => {
    //   }
    // };
    
    let community_mods =
      CommunityModeratorView::for_community(context.pool(), community_id).await?;

    // Make sure deleter is the top mod
    is_top_mod(&local_user_view, &community_mods)?;

    // Do the delete
    let community_id = data.community_id;
    let deleted = data.deleted;
    Community::update(
      context.pool(),
      community_id,
      &CommunityUpdateForm::builder()
        .deleted(Some(deleted))
        .build(),
    )
    .await
    .map_err(|e| LemmyError::from_error_message(e, "couldnt_update_community"))?;

    build_community_response(context, local_user_view, community_id).await
  }
}
