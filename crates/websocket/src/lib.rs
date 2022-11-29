#[macro_use]
extern crate strum_macros;

use crate::chat_server::ChatServer;
use actix::Addr;
use lemmy_db_schema::{source::secret::Secret, utils::DbPool};
use lemmy_utils::{
  error::LemmyError,
  rate_limit::RateLimitCell,
  settings::{structs::Settings, SETTINGS},
};
use reqwest_middleware::ClientWithMiddleware;
use serde::Serialize;

pub mod chat_server;
pub mod handlers;
pub mod messages;
pub mod routes;
pub mod send;

pub struct LemmyContext {
  pool: DbPool,
  chat_server: Addr<ChatServer>,
  client: ClientWithMiddleware,
  settings: Settings,
  secret: Secret,
  rate_limit_cell: RateLimitCell,
}

impl LemmyContext {
  pub fn create(
    pool: DbPool,
    chat_server: Addr<ChatServer>,
    client: ClientWithMiddleware,
    settings: Settings,
    secret: Secret,
    settings_updated_channel: RateLimitCell,
  ) -> LemmyContext {
    LemmyContext {
      pool,
      chat_server,
      client,
      settings,
      secret,
      rate_limit_cell: settings_updated_channel,
    }
  }
  pub fn pool(&self) -> &DbPool {
    &self.pool
  }
  pub fn chat_server(&self) -> &Addr<ChatServer> {
    &self.chat_server
  }
  pub fn client(&self) -> &ClientWithMiddleware {
    &self.client
  }
  pub fn settings(&self) -> &'static Settings {
    &SETTINGS
  }
  pub fn secret(&self) -> &Secret {
    &self.secret
  }
  pub fn settings_updated_channel(&self) -> &RateLimitCell {
    &self.rate_limit_cell
  }
}

impl Clone for LemmyContext {
  fn clone(&self) -> Self {
    LemmyContext {
      pool: self.pool.clone(),
      chat_server: self.chat_server.clone(),
      client: self.client.clone(),
      settings: self.settings.clone(),
      secret: self.secret.clone(),
      rate_limit_cell: self.rate_limit_cell.clone(),
    }
  }
}

#[derive(Serialize)]
struct WebsocketResponse<T> {
  op: String,
  data: T,
}

pub fn serialize_websocket_message<OP, Response>(
  op: &OP,
  data: &Response,
) -> Result<String, LemmyError>
where
  Response: Serialize,
  OP: ToString,
{
  let response = WebsocketResponse {
    op: op.to_string(),
    data,
  };
  Ok(serde_json::to_string(&response)?)
}

#[derive(EnumString, Display, Debug, Clone)]
pub enum UserOperation {
  Login,
  GetCaptcha,
  SaveComment,
  CreateCommentLike,
  CreateCommentReport,
  ResolveCommentReport,
  ListCommentReports,
  CreatePostLike,
  LockPost,
  StickyPost,
  MarkPostAsRead,
  SavePost,
  CreatePostReport,
  ResolvePostReport,
  ListPostReports,
  GetReportCount,
  GetUnreadCount,
  VerifyEmail,
  FollowCommunity,
  GetReplies,
  GetPersonMentions,
  MarkPersonMentionAsRead,
  MarkCommentReplyAsRead,
  GetModlog,
  BanFromCommunity,
  AddModToCommunity,
  AddAdmin,
  GetUnreadRegistrationApplicationCount,
  ListRegistrationApplications,
  ApproveRegistrationApplication,
  BanPerson,
  GetBannedPersons,
  Search,
  ResolveObject,
  MarkAllAsRead,
  SaveUserSettings,
  TransferCommunity,
  LeaveAdmin,
  PasswordReset,
  PasswordChange,
  MarkPrivateMessageAsRead,
  CreatePrivateMessageReport,
  ResolvePrivateMessageReport,
  ListPrivateMessageReports,
  UserJoin,
  PostJoin,
  CommunityJoin,
  ModJoin,
  ChangePassword,
  GetSiteMetadata,
  BlockCommunity,
  BlockPerson,
  PurgePerson,
  PurgeCommunity,
  PurgePost,
  PurgeComment,
  GetToken,
}

#[derive(EnumString, Display, Debug, Clone)]
pub enum UserOperationCrud {
  // Site
  CreateSite,
  GetSite,
  EditSite,
  // Community
  CreateCommunity,
  ListCommunities,
  GetCommunity,
  EditCommunity,
  DeleteCommunity,
  RemoveCommunity,
  // Post
  CreatePost,
  GetPost,
  GetPosts,
  EditPost,
  DeletePost,
  RemovePost,
  // Comment
  CreateComment,
  GetComment,
  GetComments,
  EditComment,
  DeleteComment,
  RemoveComment,
  // User
  Register,
  GetPersonDetails,
  DeleteAccount,
  // Private Message
  CreatePrivateMessage,
  GetPrivateMessages,
  EditPrivateMessage,
  DeletePrivateMessage,
  // Web3/PiNetwork
  Web3Register,
  Web3Login,
  PiPaymentFound,
  PiRegister,
  PiLogin,
  PiAgreeRegister,
  PiRegisterWithFee,
  PiApprove,
  PiTip,
}

pub trait OperationType {}

impl OperationType for UserOperationCrud {}

impl OperationType for UserOperation {}
