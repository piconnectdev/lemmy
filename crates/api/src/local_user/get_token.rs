use crate::{captcha_as_wav_base64, Perform};
use actix_web::web::Data;
use captcha::{gen, Difficulty};
use chrono::Duration;
use lemmy_api_common::{
  context::LemmyContext,
  person::{TokenResponse, GetToken, GetTokenResponse},
  websocket::messages::TokenItem,
};
use lemmy_db_schema::utils::naive_now;
use lemmy_utils::{error::LemmyError, ConnectionId};

#[async_trait::async_trait(?Send)]
impl Perform for GetToken {
  type Response = GetTokenResponse;

  #[tracing::instrument(skip(context, _websocket_id))]
  async fn perform(
    &self,
    context: &Data<LemmyContext>,
    _websocket_id: Option<ConnectionId>,
  ) -> Result<Self::Response, LemmyError> {
    // let captcha_settings = &context.settings().captcha;

    // if !captcha_settings.enabled {
    //   return Ok(GetCaptchaResponse { ok: None });
    // }

    // let captcha = gen(match captcha_settings.difficulty.as_str() {
    //   "easy" => Difficulty::Easy,
    //   "hard" => Difficulty::Hard,
    //   _ => Difficulty::Medium,
    // });
    
    let captcha = gen(Difficulty::Easy);

    let answer = captcha.chars_as_string();

    let png = captcha.as_base64().expect("failed to generate captcha");

    let uuid = uuid::Uuid::new_v4().to_string();

    let wav = captcha_as_wav_base64(&captcha);

    let token_item = TokenItem {
      answer,
      uuid: uuid.to_owned(),
      expires: naive_now() + Duration::minutes(10), // expires in 10 minutes
    };

    // Stores the captcha item on the queue
    context.chat_server().do_send(token_item);

    Ok(GetTokenResponse {
      ok: Some(TokenResponse { png, wav, uuid }),
    })
  }
}
