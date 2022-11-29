use crate::PerformCrud;
use actix_web::web::Data;
use lemmy_api_common::{
  post::{GetPost, GetPostResponse},
  utils::{check_private_instance, get_local_user_view_from_jwt_opt, mark_post_as_read},
};
use lemmy_db_schema::{
  aggregates::structs::{PersonPostAggregates, PersonPostAggregatesForm},
  source::{comment::Comment, local_site::LocalSite},
  traits::{Crud, DeleteableOrRemoveable},
};
use lemmy_db_views::structs::PostView;
use lemmy_db_views_actor::structs::{CommunityModeratorView, CommunityView};
use lemmy_utils::{error::LemmyError, ConnectionId};
use lemmy_websocket::{messages::GetPostUsersOnline, LemmyContext};

#[async_trait::async_trait(?Send)]
impl PerformCrud for GetPost {
  type Response = GetPostResponse;

  #[tracing::instrument(skip(context, _websocket_id))]
  async fn perform(
    &self,
    context: &Data<LemmyContext>,
    _websocket_id: Option<ConnectionId>,
  ) -> Result<GetPostResponse, LemmyError> {
    let data: &GetPost = self;
    let local_user_view =
      get_local_user_view_from_jwt_opt(data.auth.as_ref(), context.pool(), context.secret())
        .await?;
    let local_site = LocalSite::read(context.pool()).await?;

    check_private_instance(&local_user_view, &local_site)?;

    let person_id = local_user_view.map(|u| u.person.id);

    // I'd prefer fetching the post_view by a comment join, but it adds a lot of boilerplate
    let post_id = if let Some(id) = data.id {
      id
    } else if let Some(comment_id) = data.comment_id {
      Comment::read(context.pool(), comment_id)
        .await
        .map_err(|e| LemmyError::from_error_message(e, "couldnt_find_post"))?
        .post_id
    } else {
      Err(LemmyError::from_message("couldnt_find_post"))?
    };

    let mut post_view = PostView::read(context.pool(), post_id, person_id)
      .await
      .map_err(|e| LemmyError::from_error_message(e, "couldnt_find_post"))?;

    // Mark the post as read
    let post_id = post_view.post.id;
    if let Some(person_id) = person_id {
      mark_post_as_read(person_id, post_id, context.pool()).await?;
    }

    // Necessary for the sidebar subscribed
    let community_id = post_view.community.id;
    let mut community_view = CommunityView::read(context.pool(), community_id, person_id)
      .await
      .map_err(|e| LemmyError::from_error_message(e, "couldnt_find_community"))?;

    // Insert into PersonPostAggregates
    // to update the read_comments count
    if let Some(person_id) = person_id {
      let read_comments = post_view.counts.comments;
      let person_post_agg_form = PersonPostAggregatesForm {
        person_id,
        post_id,
        read_comments,
        ..PersonPostAggregatesForm::default()
      };
      PersonPostAggregates::upsert(context.pool(), &person_post_agg_form)
        .await
        .map_err(|e| LemmyError::from_error_message(e, "couldnt_find_post"))?;
    }

    // Blank out deleted or removed info for non-logged in users
    if person_id.is_none() {
      if post_view.post.deleted || post_view.post.removed {
        post_view.post = post_view.post.blank_out_deleted_or_removed_info();
      }

      if community_view.community.deleted || community_view.community.removed {
        community_view.community = community_view.community.blank_out_deleted_or_removed_info();
      }
    }

    let moderators = CommunityModeratorView::for_community(context.pool(), community_id).await?;

    let online = context
      .chat_server()
      .send(GetPostUsersOnline { post_id })
      .await
      .unwrap_or(1);

    // Return the jwt
    Ok(GetPostResponse {
      post_view,
      community_view,
      moderators,
      online,
    })
  }
}
