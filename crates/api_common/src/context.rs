use crate::websocket::chat_server::ChatServer;
use lemmy_db_schema::{source::secret::Secret, utils::DbPool};
use lemmy_utils::{
  rate_limit::RateLimitCell,
  settings::{structs::Settings, SETTINGS},
};
use reqwest_middleware::ClientWithMiddleware;
use std::sync::Arc;

#[derive(Clone)]
pub struct LemmyContext {
  pool: DbPool,
  client: Arc<ClientWithMiddleware>,
  secret: Arc<Secret>,
  rate_limit_cell: RateLimitCell,
  chat_server: Arc<ChatServer>,
}

impl LemmyContext {
  pub fn create(
    pool: DbPool,
    client: ClientWithMiddleware,
    secret: Secret,
    rate_limit_cell: RateLimitCell,
    chat_server: Arc<ChatServer>,
  ) -> LemmyContext {
    LemmyContext {
      pool,
      client: Arc::new(client),
      secret: Arc::new(secret),
      rate_limit_cell,
      chat_server,
    }
  }
  pub fn pool(&self) -> &DbPool {
    &self.pool
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
  pub fn chat_server(&self) -> &Arc<ChatServer> {
    &self.chat_server
  }
}
