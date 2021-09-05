use crate::Perform;
use actix_web::web::Data;
use lemmy_api_common::{get_local_user_view_from_jwt, websocket::*};
use lemmy_utils::{ConnectionId, LemmyError};
use lemmy_websocket::{
  messages::{JoinCommunityRoom, JoinModRoom, JoinPostRoom, JoinUserRoom},
  LemmyContext,
};
use lemmy_db_schema::{CommunityId,};
use uuid::Uuid;
#[async_trait::async_trait(?Send)]
impl Perform for UserJoin {
  type Response = UserJoinResponse;

  async fn perform(
    &self,
    context: &Data<LemmyContext>,
    websocket_id: Option<ConnectionId>,
  ) -> Result<UserJoinResponse, LemmyError> {
    let data: &UserJoin = self;
    let local_user_view = get_local_user_view_from_jwt(&data.auth, context.pool()).await?;

    if let Some(ws_id) = websocket_id {
      context.chat_server().do_send(JoinUserRoom {
        local_user_id: local_user_view.local_user.id,
        id: ws_id,
      });
    }

    Ok(UserJoinResponse { joined: true })
  }
}

#[async_trait::async_trait(?Send)]
impl Perform for CommunityJoin {
  type Response = CommunityJoinResponse;

  async fn perform(
    &self,
    context: &Data<LemmyContext>,
    websocket_id: Option<ConnectionId>,
  ) -> Result<CommunityJoinResponse, LemmyError> {
    let data: &CommunityJoin = self;

    if let Some(ws_id) = websocket_id {
      // let community_id = match Uuid::parse_str(&data.community_id.clone()) {
      //   Ok(uid) => {
      //       CommunityId(uid)
      //   },
      //   Err(e) => {
      //     let xid = Uuid::parse_str("00000000-0000-0000-0000-000000000000").unwrap();
      //     CommunityId(xid)
      //   }
      // };

      let community_id = match &data.community_id {
        Some(id) => {
          let uuid = Uuid::parse_str(&id.clone());
          match uuid {
            Ok(u) => CommunityId(u),
            Err(e) => {
              let xid = Uuid::parse_str("00000000-0000-0000-0000-000000000000").unwrap();
              CommunityId(xid)
            }
          }
        },
        None => {
          let xid = Uuid::parse_str("00000000-0000-0000-0000-000000000000").unwrap();
          CommunityId(xid)
        }
      };
      context.chat_server().do_send(JoinCommunityRoom {
        community_id: community_id,
        id: ws_id,
      });
    }

    Ok(CommunityJoinResponse { joined: true })
  }
}

#[async_trait::async_trait(?Send)]
impl Perform for ModJoin {
  type Response = ModJoinResponse;

  async fn perform(
    &self,
    context: &Data<LemmyContext>,
    websocket_id: Option<ConnectionId>,
  ) -> Result<ModJoinResponse, LemmyError> {
    let data: &ModJoin = self;

    if let Some(ws_id) = websocket_id {
      context.chat_server().do_send(JoinModRoom {
        community_id: data.community_id,
        id: ws_id,
      });
    }

    Ok(ModJoinResponse { joined: true })
  }
}

#[async_trait::async_trait(?Send)]
impl Perform for PostJoin {
  type Response = PostJoinResponse;

  async fn perform(
    &self,
    context: &Data<LemmyContext>,
    websocket_id: Option<ConnectionId>,
  ) -> Result<PostJoinResponse, LemmyError> {
    let data: &PostJoin = self;

    if let Some(ws_id) = websocket_id {
      context.chat_server().do_send(JoinPostRoom {
        post_id: data.post_id,
        id: ws_id,
      });
    }

    Ok(PostJoinResponse { joined: true })
  }
}
