use crate::{fetcher::webfinger::webfinger_resolve_actor, ActorType};
use activitypub_federation::traits::ApubObject;
use itertools::Itertools;
use lemmy_api_common::context::LemmyContext;
use lemmy_db_schema::traits::ApubActor;
use lemmy_utils::error::LemmyError;

pub mod post_or_comment;
pub mod search;
pub mod user_or_community;
pub mod webfinger;

/// Resolve actor identifier (eg `!news@example.com`) from local database to avoid network requests.
/// This only works for local actors, and remote actors which were previously fetched (so it doesnt
/// trigger any new fetch).
#[tracing::instrument(skip_all)]
pub async fn resolve_actor_identifier<Actor, DbActor>(
  identifier: &str,
  context: &LemmyContext,
  include_deleted: bool,
) -> Result<DbActor, LemmyError>
where
  Actor: ApubObject<DataType = LemmyContext, Error = LemmyError>
    + ApubObject<DbType = DbActor>
    + ActorType
    + Send
    + 'static,
  for<'de2> <Actor as ApubObject>::ApubType: serde::Deserialize<'de2>,
  DbActor: ApubActor + Send + 'static,
{
  // remote actor
  if identifier.contains('@') {
    let (name, domain) = identifier
      .splitn(2, '@')
      .collect_tuple()
      .expect("invalid query");
    let name = name.to_string();
    let domain = format!("{}://{}", context.settings().get_protocol_string(), domain);
    let actor = DbActor::read_from_name_and_domain(context.pool(), &name, &domain).await;
    if actor.is_ok() {
      Ok(actor?)
    } else {
      // Fetch the actor from its home instance using webfinger
      let id = webfinger_resolve_actor::<Actor>(identifier, true, context, &mut 0).await?;
      let actor: DbActor = DbActor::read_from_apub_id(context.pool(), &id)
        .await?
        .expect("actor exists as we fetched just before");
      Ok(actor)
    }
  }
  // local actor
  else {
    let identifier = identifier.to_string();
    Ok(DbActor::read_from_name(context.pool(), &identifier, include_deleted).await?)
  }
}
