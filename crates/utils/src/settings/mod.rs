use crate::{
  error::LemmyError,
  location_info,
  settings::structs::{PictrsConfig, Settings},
};
use anyhow::{anyhow, Context};
use deser_hjson::from_str;
use once_cell::sync::Lazy;
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use regex::Regex;
use std::{env, fs, io::Error};
use ethsign::SecretKey;
use rustc_hex::{FromHex};
pub mod structs;

use structs::DatabaseConnection;

static DEFAULT_CONFIG_FILE: &str = "config/config.hjson";

pub static SETTINGS: Lazy<Settings> = Lazy::new(|| {
  Settings::init().expect("Failed to load settings file, see documentation (https://join-lemmy.org/docs/en/administration/configuration.html)")
});
static WEBFINGER_REGEX: Lazy<Regex> = Lazy::new(|| {
  Regex::new(&format!(
    "^acct:([a-zA-Z0-9_]{{3,}})@{}$",
    SETTINGS.hostname
  ))
  .expect("compile webfinger regex")
});

pub static SECRETKEY: Lazy<SecretKey> =
  Lazy::new(|| {
    let str_key= match &SETTINGS.secret_key {
        Some(str) => {str},
        None => "4d5db4107d237df6a3d58ee5f70ae63d73d7658d4026f2eefd2f204c81682cb7",
    };
    let secret: Vec<u8> = str_key
    .from_hex()
    .unwrap();
    let key = SecretKey::from_raw(&secret).unwrap();
    return key;
  });

impl Settings {
  /// Reads config from configuration file.
  ///
  /// Note: The env var `LEMMY_DATABASE_URL` is parsed in
  /// `lemmy_db_schema/src/lib.rs::get_database_url_from_env()`
  /// Warning: Only call this once.
  pub(crate) fn init() -> Result<Self, LemmyError> {
    // Read the config file
    let config = from_str::<Settings>(&Self::read_config_file()?)?;

    if config.hostname == "unset" {
      return Err(anyhow!("Hostname variable is not set!").into());
    }

    Ok(config)
  }

  pub fn get_database_url(&self) -> String {
    match &self.database.connection {
      DatabaseConnection::Uri { uri } => uri.clone(),
      DatabaseConnection::Parts(parts) => {
        format!(
          "postgres://{}:{}@{}:{}/{}",
          utf8_percent_encode(&parts.user, NON_ALPHANUMERIC),
          utf8_percent_encode(&parts.password, NON_ALPHANUMERIC),
          parts.host,
          parts.port,
          utf8_percent_encode(&parts.database, NON_ALPHANUMERIC),
        )
      }
    }
  }

  fn get_config_location() -> String {
    env::var("LEMMY_CONFIG_LOCATION").unwrap_or_else(|_| DEFAULT_CONFIG_FILE.to_string())
  }

  fn read_config_file() -> Result<String, Error> {
    fs::read_to_string(Self::get_config_location())
  }

  /// Returns either "http" or "https", depending on tls_enabled setting
  pub fn get_protocol_string(&self) -> &'static str {
    if self.tls_enabled {
      "https"
    } else {
      "http"
    }
  }

  /// Returns something like `http://localhost` or `https://lemmy.ml`,
  /// with the correct protocol and hostname.
  pub fn get_protocol_and_hostname(&self) -> String {
    format!("{}://{}", self.get_protocol_string(), self.hostname)
  }

  /// When running the federation test setup in `api_tests/` or `docker/federation`, the `hostname`
  /// variable will be like `lemmy-alpha:8541`. This method removes the port and returns
  /// `lemmy-alpha` instead. It has no effect in production.
  pub fn get_hostname_without_port(&self) -> Result<String, anyhow::Error> {
    Ok(
      (*self
        .hostname
        .split(':')
        .collect::<Vec<&str>>()
        .first()
        .context(location_info!())?)
      .to_string(),
    )
  }

  pub fn webfinger_regex(&self) -> Regex {
    WEBFINGER_REGEX.clone()
  }

  pub fn pictrs_config(&self) -> Result<PictrsConfig, LemmyError> {
    self
      .pictrs
      .clone()
      .ok_or_else(|| anyhow!("images_disabled").into())
  }

  pub fn pi_seed(&self) -> String {
    self.pinetwork.pi_seed.to_owned().unwrap_or_default()
  }
  pub fn pi_key(&self) -> String {
    self.pinetwork.pi_key.to_owned().unwrap_or_default()
  }
  pub fn pi_api_host(&self) -> String {
    self.pinetwork.pi_api_host.to_owned().unwrap_or_default()
  }
  pub fn pi_horizon_host(&self) -> String {
    self.pinetwork.pi_api_host.to_owned().unwrap_or_default()
  }
  pub fn web3_admin(&self) -> String {
    self.web3.admin.to_owned().unwrap_or_default()
  }
  pub fn web3_signer(&self) -> String {
    self.web3.signer.to_owned().unwrap_or_default()
  }
  pub fn web3_signer_key(&self) -> String {
    self.web3.signer_key.to_owned().unwrap_or_default()
  }
}
