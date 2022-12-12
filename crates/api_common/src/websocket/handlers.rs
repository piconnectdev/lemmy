use crate::websocket::{chat_server::ChatServer, structs::{CaptchaItem, TokenItem, PiTokenItem}};
use actix_ws::Session;
use lemmy_db_schema::{
  newtypes::{CommunityId, PostId},
  utils::naive_now,
};
use lemmy_utils::{error::LemmyError, ConnectionId};
use rand::Rng;
use crate::pipayment::PiUserDto;
impl ChatServer {
  /// Handler for Connect message.
  ///
  /// Register new session and assign unique id to this session
  pub fn handle_connect(&self, session: Session) -> Result<ConnectionId, LemmyError> {
    let mut inner = self.inner()?;
    // register session with random id
    let id = inner.rng.gen::<usize>();

    inner.sessions.insert(id, session);
    Ok(id)
  }

  /// Handler for Disconnect message.
  pub fn handle_disconnect(&self, connection_id: &ConnectionId) -> Result<(), LemmyError> {
    let mut inner = self.inner()?;
    // Remove connections from sessions and all 3 scopes
    if inner.sessions.remove(connection_id).is_some() {
      for sessions in inner.user_rooms.values_mut() {
        sessions.remove(connection_id);
      }

      for sessions in inner.post_rooms.values_mut() {
        sessions.remove(connection_id);
      }

      for sessions in inner.community_rooms.values_mut() {
        sessions.remove(connection_id);
      }
    }
    Ok(())
  }

  pub fn get_users_online(&self) -> Result<usize, LemmyError> {
    Ok(self.inner()?.sessions.len())
  }

  pub fn get_post_users_online(&self, post_id: PostId) -> Result<usize, LemmyError> {
    if let Some(users) = self.inner()?.post_rooms.get(&post_id) {
      Ok(users.len())
    } else {
      Ok(0)
    }
  }

  pub fn get_community_users_online(&self, community_id: CommunityId) -> Result<usize, LemmyError> {
    if let Some(users) = self.inner()?.community_rooms.get(&community_id) {
      Ok(users.len())
    } else {
      Ok(0)
    }
  }

  pub fn add_captcha(&self, captcha: CaptchaItem) -> Result<(), LemmyError> {
    self.inner()?.captchas.push(captcha);
    Ok(())
  }

  pub fn check_captcha(&self, uuid: String, answer: String) -> Result<bool, LemmyError> {
    let mut inner = self.inner()?;
    // Remove all the ones that are past the expire time
    inner.captchas.retain(|x| x.expires.gt(&naive_now()));

    let check = inner
      .captchas
      .iter()
      .any(|r| r.uuid == uuid && r.answer.to_lowercase() == answer.to_lowercase());

    // Remove this uuid so it can't be re-checked (Checks only work once)
    inner.captchas.retain(|x| x.uuid != uuid);

    Ok(check)
  }

  pub fn add_token(&self, captcha: TokenItem) -> Result<(), LemmyError> {
    self.inner()?.tokens.push(captcha);
    Ok(())
  }

  pub fn check_token(&self, uuid: String, answer: String) -> Result<bool, LemmyError> {
    let mut inner = self.inner()?;
    // Remove all the ones that are past the expire time
    inner.tokens.retain(|x| x.expires.gt(&naive_now()));

    // let check = inner
    //   .tokens
    //   .iter()
    //   .any(|r| r.uuid == uuid && r.answer.to_lowercase() == answer.to_lowercase());

      let check = inner.tokens.iter().any(|r| r.uuid == uuid);
    // Remove this uuid so it can't be re-checked (Checks only work once)
    inner.tokens.retain(|x| x.uuid != uuid);

    Ok(check)
  }
  
  pub fn add_pi_token(&self, captcha: PiTokenItem) -> Result<(), LemmyError> {
    self.inner()?.piTokens.push(captcha);
    Ok(())
  }

  pub fn check_pi_token(&self, uuid: String, answer: String) -> Result<Option<PiUserDto>, LemmyError> {
    let mut inner = self.inner()?;
    // Remove all the ones that are past the expire time
    inner.piTokens.retain(|x| x.expires.gt(&naive_now()));

    // let check = inner
    //   .tokens
    //   .iter()
    //   .any(|r| r.uuid == uuid && r.answer.to_lowercase() == answer.to_lowercase());

    // // Remove this uuid so it can't be re-checked (Checks only work once)
    // inner.tokens.retain(|x| x.uuid != uuid);

    // Ok(check)
    let mut iter = inner.piTokens.iter();
    //let dto = self.piTokens.iter().any(|r| r.uuid == msg.uuid,r);
    match iter.find(|&x| x.uuid == uuid){
      Some(dto) => Ok(Some(dto.answer.clone())),
      None => Ok(None),
    }
  }

}

// impl Handler<TokenItem> for ChatServer {
//   type Result = ();

//   fn handle(&mut self, msg: TokenItem, _: &mut Context<Self>) {
//     self.tokens.push(msg);
//   }
// }

// impl Handler<CheckToken> for ChatServer {
//   type Result = bool;

//   fn handle(&mut self, msg: CheckToken, _: &mut Context<Self>) -> Self::Result {
//     // Remove all the ones that are past the expire time
//     self.tokens.retain(|x| x.expires.gt(&naive_now()));

//     let check = self.tokens.iter().any(|r| r.uuid == msg.uuid);

//     // Remove this uuid so it can't be re-checked (Checks only work once)
//     self.tokens.retain(|x| x.uuid != msg.uuid);

//     check
//   }
// }

// impl Handler<PiTokenItem> for ChatServer {
//   type Result = ();

//   fn handle(&mut self, msg: PiTokenItem, _: &mut Context<Self>) {
//     self.piTokens.push(msg);
//   }
// }

// impl Handler<CheckPiToken> for ChatServer {
//   type Result = Option<PiUserDto>;

//   fn handle(&mut self, msg: CheckPiToken, _: &mut Context<Self>) -> Self::Result {
//     // Remove all the ones that are past the expire time
//     self.piTokens.retain(|x| x.expires.gt(&naive_now()));

//     let mut iter = self.piTokens.iter();
//     //let dto = self.piTokens.iter().any(|r| r.uuid == msg.uuid,r);
//     match iter.find(|&x| x.uuid == msg.uuid){
//       Some(dto) => Some(dto.answer.clone()),
//       None => None,
//     }
//   }
//}