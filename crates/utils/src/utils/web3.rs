use crate::{settings::SECRETKEY};
use ethsign::{KeyFile, Protected, PublicKey, SecretKey, Signature};
use hex::FromHex;
use web3::signing::{keccak256, recover};

pub fn eth_message(message: String) -> [u8; 32] {
    keccak256(
      format!(
        "{}{}{}",
        "\x19Ethereum Signed Message:\n",
        message.len(),
        message
      )
      .as_bytes(),
    )
  }
  
  pub fn eth_verify(account: String, data: String, signature: String) -> bool {
    let message = eth_message(data.clone());
    // Remove 0x prefix
    let signature = &signature[2..];
    let signature_bytes = hex::decode(signature).unwrap();
    let recovery_id = signature_bytes[64] as i32 - 27;
  
    let pubkey = recover(&message, &signature_bytes[..64], recovery_id);
    if !pubkey.is_ok() {
      return false;
    }
    // assert!(pubkey.is_ok());
    let pubkey = pubkey.unwrap();
    let pubkey = format!("{:02X?}", pubkey);
    if account != pubkey {
      return false;
      //return Some(pubkey);
    }
    //println!("eth_verify: {} {} {:?} {}", account, hex::encode(message), pubkey, signature);
    //return Some(pubkey);
    true
  }
  
  pub fn eth_sign_message(message: String) -> Option<String> {
  
    if !crate::settings::SETTINGS.web3.enabled {
      return None;
    }
    
    if !crate::settings::SETTINGS.secret_key.is_some() {
      return None;
    }
  
    let key = SECRETKEY.clone();
    let message_hash = eth_message(message.clone());
    //println!("\r\nSign to ETH-msg-hash {} of {} \r\n", hex::encode(message_hash), message.clone());
    let signature = key.sign(&message_hash).unwrap();
    let sig = format!(
      "0x{}{}{:02X?}",
      hex::encode(signature.r),
      hex::encode(signature.s),
      signature.v + 27
    );
    Some(sig)
  }

#[cfg(test)]
mod tests {
  use crate::{settings::SECRETKEY};
  use ethsign::SecretKey;
  use rustc_hex::{FromHex, ToHex};
  use url::Url;
  use web3::signing::recover;
  use super::{eth_verify, eth_message};
 

  #[test]
  fn test_recover() {
    // https://www.shawntabrizi.com/ETH-Sign-and-Verify/
    
    let account = "0x63f9a92d8d61b48a9fff8d58080425a3012d05c8".to_string();
    let message = "0x63f9a92d8d61b48a9fff8d58080425a3012d05c8igwyk4r1o7o".to_string();
    let signature = "0x382a3e04daf88f322730f6a2972475fc5646ea8c4a7f3b5e83a90b10ba08a7364cd2f55348f2b6d210fbed7fc485abf19ecb2f3967e410d6349dd7dd1d4487751b".to_string();
    //let message = eth_message(message);
    //let signature = hex::decode("382a3e04daf88f322730f6a2972475fc5646ea8c4a7f3b5e83a90b10ba08a7364cd2f55348f2b6d210fbed7fc485abf19ecb2f3967e410d6349dd7dd1d4487751b").unwrap();
    eth_verify(account.clone(), message.clone(), signature);

    let secret: Vec<u8> = "4d5db4107d237df6a3d58ee5f70ae63d73d7658d4026f2eefd2f204c81682cb7"
      .from_hex()
      .unwrap();
    
    //let key = SECRETKEY.clone();
    let key = SecretKey::from_raw(&secret).unwrap();
    let message_hash = eth_message(message.clone());
    //println!("\r\nSign to ETH-msg-hash {} of {} \r\n", hex::encode(message_hash), message.clone());
    let signature = key.sign(&message_hash).unwrap();
    let sig = format!(
      "0x{}{}{:02X?}",
      hex::encode(signature.r),
      hex::encode(signature.s),
      signature.v + 27
    );
    let add = format!("0x{}", hex::encode(key.public().address()));
    //println!("signature: {} {}", add, sig);
    eth_verify(add, message.clone(), sig);
    
    //let updated: Option<chrono::NaiveDateTime>
    //let updated: Option<chrono::NaiveDateTime> = None;//chrono::prelude::Utc::now().naive_utc();
    //println!("NaiveDateTime: {:?}", updated);

  }
}

