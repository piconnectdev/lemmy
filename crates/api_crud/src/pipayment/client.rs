use lemmy_utils::{
  apub::generate_actor_keypair,
  claims::Claims,
  pipayment::PiPaymentDto,
  pipayment::*,
  request::*,
  settings::structs::Settings,
  utils::{check_slurs, is_valid_username},
  ApiError, ConnectionId, LemmyError,
};
use reqwest::Client;

pub async fn pi_payment(client: &Client, id: &String) -> Result<PiPaymentDto, LemmyError> {
  let fetch_url = format!("{}/payments/{}", Settings::get().pi_api_host(), id);

  let response = retry(|| {
    client
      .get(&fetch_url)
      .header("Authorization", format!("Key {}", Settings::get().pi_key()))
      .send()
  })
  .await?;

  let res: PiPaymentDto = response
    .json::<PiPaymentDto>()
    .await
    .map_err(|e| RecvError(e.to_string()))?;
  Ok(res)
}

pub async fn pi_approve(client: &Client, id: &String) -> Result<PiPaymentDto, LemmyError> {
  let fetch_url = format!("{}/payments/{}/approve", Settings::get().pi_api_host(), id);

  let response = retry(|| {
    client
      .post(&fetch_url)
      .header("Authorization", format!("Key {}", Settings::get().pi_key()))
      .send()
  })
  .await?;

  let res: PiPaymentDto = response
    .json::<PiPaymentDto>()
    .await
    .map_err(|e| RecvError(e.to_string()))?;
  Ok(res)
}

pub async fn pi_complete(
  client: &Client,
  id: &String,
  txid_: &String,
) -> Result<PiPaymentDto, LemmyError> {
  let fetch_url = format!("{}/payments/{}/complete", Settings::get().pi_api_host(), id);

  let r = TxRequest {
    txid: txid_.to_owned(),
  };
  let response = retry(|| {
    client
      .post(&fetch_url)
      .header("Authorization", format!("Key {}", Settings::get().pi_key()))
      .json(&r)
      .send()
  })
  .await?;

  let res: PiPaymentDto = response
    .json::<PiPaymentDto>()
    .await
    .map_err(|e| RecvError(e.to_string()))?;
  Ok(res)
}
