use crate::{
  api::listing_type_with_default, fetcher::resolve_actor_identifier,
  objects::community::ApubCommunity,
};
use activitypub_federation::config::Data;
use actix_web::web::{Json, Query};
use lemmy_api_common::{
  context::LemmyContext,
  post::{GetPosts, GetPostsResponse},
  utils::{check_private_instance, is_mod_or_admin_opt, local_user_view_from_jwt_opt},
};
use lemmy_db_schema::source::{community::Community, local_site::LocalSite};
use lemmy_db_views::post_view::PostQuery;
use lemmy_utils::error::LemmyError;

#[tracing::instrument(skip(context))]
pub async fn list_posts(
  data: Query<GetPosts>,
  context: Data<LemmyContext>,
) -> Result<Json<GetPostsResponse>, LemmyError> {
  let local_user_view = local_user_view_from_jwt_opt(data.auth.as_ref(), &context).await;
  let local_site = LocalSite::read(context.pool()).await?;

  check_private_instance(&local_user_view, &local_site)?;

  let sort = data.sort;

  let page = data.page;
  let limit = data.limit;
  /*
  let community_actor_id = if let Some(name) = &data.community_name {
      resolve_actor_identifier::<ApubCommunity, Community>(name, context, true)
        .await
        .ok()
        .map(|c| c.id)
    } else {
      data.community_id
    };
  */

  let community_id = if let Some(name) = &data.community_name {
    Some(resolve_actor_identifier::<ApubCommunity, Community>(name, &context, &None, true).await?)
      .map(|c| c.id)
  } else {
    data.community_id
  };
  let saved_only = data.saved_only;

  let listing_type = listing_type_with_default(data.type_, &local_site, community_id)?;

  let is_mod_or_admin = is_mod_or_admin_opt(context.pool(), local_user_view.as_ref(), community_id)
    .await
    .is_ok();

  let posts = PostQuery::builder()
    .pool(context.pool())
    .local_user(local_user_view.map(|l| l.local_user).as_ref())
    .listing_type(Some(listing_type))
    .sort(sort)
    .community_id(community_id)
    .saved_only(saved_only)
    .page(page)
    .limit(limit)
    .is_mod_or_admin(Some(is_mod_or_admin))
    .build()
    .list()
    .await
    .map_err(|e| LemmyError::from_error_message(e, "couldnt_get_posts"))?;

  Ok(Json(GetPostsResponse { posts }))
}

/*
#[async_trait::async_trait(?Send)]
impl PerformCrud for GetPosts {
  type Response = GetPostsResponse;

  async fn perform(
    &self,
    context: &Data<LemmyContext>,
  ) -> Result<GetPostsResponse, LemmyError> {
    let data: &GetPosts = self;
    let local_user_view = get_local_user_view_from_jwt_opt(&data.auth, context.pool()).await?;

    let person_id = local_user_view.to_owned().map(|l| l.person.id);

    let show_nsfw = local_user_view.as_ref().map(|t| t.local_user.show_nsfw);
    let show_bot_accounts = local_user_view
      .as_ref()
      .map(|t| t.local_user.show_bot_accounts);
    let show_read_posts = local_user_view
      .as_ref()
      .map(|t| t.local_user.show_read_posts);

    let sort: Option<SortType> = from_opt_str_to_opt_enum(&data.sort);
    let listing_type: Option<ListingType> = from_opt_str_to_opt_enum(&data.type_);

    let page = data.page;
    let limit = data.limit;
    let mut community_name = data.community_name.to_owned();
    let community_id =  match &data.community_id {
        Some(cid) => {
          /// TODO: UUID check
          let uuid = Uuid::parse_str(cid);
          match uuid {
              Ok(uid) => Some(CommunityId(uid)),
              Err(e) => {
                community_name = data.community_id.clone();
                None
              }
          }
        },
        None => None
    };
    let community_actor_id = data
      .community_name
      .as_ref()
      .map(|t| build_actor_id_from_shortname(EndpointType::Community, t).ok())
      .unwrap_or(None);
    let saved_only = data.saved_only;

    let mut posts = blocking(context.pool(), move |conn| {
      PostQueryBuilder::create(conn)
        .listing_type(listing_type)
        .sort(sort)
        .show_nsfw(show_nsfw)
        .show_bot_accounts(show_bot_accounts)
        .show_read_posts(show_read_posts)
        .community_id(community_id)
        .community_actor_id(community_actor_id)
        .saved_only(saved_only)
        .my_person_id(person_id)
        .page(page)
        .limit(limit)
        .list()
    })
    .await?
    .map_err(|_| LemmyError::from_message("couldnt_get_posts"))?;

    // Blank out deleted or removed info
    for pv in posts
      .iter_mut()
      .filter(|p| p.post.deleted || p.post.removed)
    {
      pv.post = pv.to_owned().post.blank_out_deleted_or_removed_info();
    }

    Ok(GetPostsResponse { posts })
  }
}
*/
