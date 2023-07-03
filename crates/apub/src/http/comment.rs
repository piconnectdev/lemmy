use crate::{
  http::{create_apub_response, create_apub_tombstone_response, err_object_not_local},
  objects::comment::ApubComment,
};
use activitypub_federation::{config::Data, traits::Object};
use actix_web::{web::Path, HttpResponse};
use lemmy_api_common::context::LemmyContext;
use lemmy_db_schema::{newtypes::CommentId, source::comment::Comment, traits::Crud};
use lemmy_utils::error::LemmyError;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub(crate) struct CommentQuery {
  comment_id: String,
}

/// Return the ActivityPub json representation of a local comment over HTTP.
#[tracing::instrument(skip_all)]
pub(crate) async fn get_apub_comment(
  info: Path<CommentQuery>,
  context: Data<LemmyContext>,
) -> Result<HttpResponse, LemmyError> {
  // TODO: DinhHa 
  // Uuid::parse_str("00000000-0000-0000-0000-000000000000").unwrap()
  //let id = CommentId(info.comment_id.parse::<i32>()?);
  let uuid = Uuid::parse_str(&info.comment_id.clone()).unwrap();
  let id = CommentId(uuid);
  let comment: ApubComment = Comment::read(context.pool(), id).await?.into();
  if !comment.local {
    return Err(err_object_not_local());
  }

  if !comment.deleted && !comment.removed {
    create_apub_response(&comment.into_json(&context).await?)
  } else {
    create_apub_tombstone_response(comment.ap_id.clone())
  }
}
