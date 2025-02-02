use doku::Document;
use ethsign::SecretKey;
use serde::{Deserialize, Serialize};
use std::net::{IpAddr, Ipv4Addr};
use url::Url;

#[derive(Debug, Deserialize, Serialize, Clone, SmartDefault, Document)]
#[serde(default)]
pub struct Settings {
  /// settings related to the postgresql database
  #[default(Default::default())]
  pub database: DatabaseConfig,
  /// Settings related to activitypub federation
  /// Pictrs image server configuration.
  #[default(Some(Default::default()))]
  pub(crate) pictrs: Option<PictrsConfig>,
  /// Email sending configuration. All options except login/password are mandatory
  #[default(None)]
  #[doku(example = "Some(Default::default())")]
  pub email: Option<EmailConfig>,
  /// Parameters for automatic configuration of new instance (only used at first start)
  #[default(None)]
  #[doku(example = "Some(Default::default())")]
  pub setup: Option<SetupConfig>,
  /// the domain name of your instance (mandatory)
  #[default("unset")]
  #[doku(example = "example.com")]
  pub hostname: String,
  /// Address where lemmy should listen for incoming requests
  #[default(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)))]
  #[doku(as = "String")]
  pub bind: IpAddr,
  /// Port where lemmy should listen for incoming requests
  #[default(8536)]
  pub port: u16,
  /// Whether the site is available over TLS. Needs to be true for federation to work.
  #[default(true)]
  pub tls_enabled: bool,
  /// Set the URL for opentelemetry exports. If you do not have an opentelemetry collector, do not set this option
  #[default(None)]
  #[doku(skip)]
  pub opentelemetry_url: Option<Url>,

  /// The number of activitypub federation workers that can be in-flight concurrently
  #[default(0)]
  pub worker_count: usize,
  /// The number of activitypub federation retry workers that can be in-flight concurrently
  #[default(0)]
  pub retry_count: usize,

  // Prometheus configuration.
  #[default(None)]
  #[doku(example = "Some(Default::default())")]
  pub prometheus: Option<PrometheusConfig>,

  #[default(PiNetworkConfig::default())]
  pub pinetwork: PiNetworkConfig,

  #[default(Web3Config::default())]
  pub web3: Web3Config,

  /// Whether the site is open for web3 registration.
  #[default(true)]
  pub open_enabled: bool,

  /// Whether the site is open for web3 registration.
  #[default(true)]
  pub web3_enabled: bool,

  /// Whether the site is open for pi network registration.
  #[default(false)]
  pub pi_enabled: bool,

  /// Whether the site is hide pi network account.
  #[default(false)]
  pub pi_hide_account: bool,

  /// Whether the site is open for web3 registration.
  #[default(false)]
  pub sign_enabled: bool,

  #[default(None)]
  pub secret_key: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, SmartDefault, Document)]
#[serde(default, deny_unknown_fields)]
pub struct PictrsConfig {
  /// Address where pictrs is available (for image hosting)
  #[default(Url::parse("http://localhost:8080").expect("parse pictrs url"))]
  #[doku(example = "http://localhost:8080")]
  pub url: Url,

  /// Set a custom pictrs API key. ( Required for deleting images )
  #[default(None)]
  pub api_key: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, SmartDefault, Document)]
#[serde(default)]
pub struct DatabaseConfig {
  #[serde(flatten, default)]
  pub connection: DatabaseConnection,

  /// Maximum number of active sql connections
  #[default(5)]
  pub pool_size: usize,
}

#[derive(Debug, Deserialize, Serialize, Clone, SmartDefault, Document)]
#[serde(untagged)]
pub enum DatabaseConnection {
  /// Configure the database by specifying a URI
  ///
  /// This is the preferred method to specify database connection details since
  /// it is the most flexible.
  Uri {
    /// Connection URI pointing to a postgres instance
    ///
    /// This example uses peer authentication to obviate the need for creating,
    /// configuring, and managing passwords.
    ///
    /// For an explanation of how to use connection URIs, see [here][0] in
    /// PostgreSQL's documentation.
    ///
    /// [0]: https://www.postgresql.org/docs/current/libpq-connect.html#id-1.7.3.8.3.6
    #[doku(example = "postgresql:///lemmy?user=lemmy&host=/var/run/postgresql")]
    uri: String,
  },

  /// Configure the database by specifying parts of a URI
  ///
  /// Note that specifying the `uri` field should be preferred since it provides
  /// greater control over how the connection is made. This merely exists for
  /// backwards-compatibility.
  #[default]
  Parts(DatabaseConnectionParts),
}

#[derive(Debug, Deserialize, Serialize, Clone, SmartDefault, Document)]
#[serde(default)]
pub struct DatabaseConnectionParts {
  /// Username to connect to postgres
  #[default("wepi")]
  pub(super) user: String,
  /// Password to connect to postgres
  #[default("password")]
  pub password: String,
  #[default("localhost")]
  /// Host where postgres is running
  pub host: String,
  /// Port where postgres can be accessed
  #[default(5432)]
  pub(super) port: i32,
  /// Name of the postgres database for lemmy
  #[default("wepi")]
  pub(super) database: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, Document, SmartDefault)]
#[serde(deny_unknown_fields)]
pub struct EmailConfig {
  /// Hostname and port of the smtp server
  #[doku(example = "localhost:25")]
  pub smtp_server: String,
  /// Login name for smtp server
  pub smtp_login: Option<String>,
  /// Password to login to the smtp server
  pub smtp_password: Option<String>,
  #[doku(example = "noreply@example.com")]
  /// Address to send emails from, eg "noreply@your-instance.com"
  pub smtp_from_address: String,
  /// Whether or not smtp connections should use tls. Can be none, tls, or starttls
  #[default("none")]
  #[doku(example = "none")]
  pub tls_type: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, SmartDefault, Document)]
#[serde(deny_unknown_fields)]
pub struct SetupConfig {
  /// Username for the admin user
  #[doku(example = "admin")]
  pub admin_username: String,
  /// Password for the admin user. It must be at least 10 characters.
  #[doku(example = "tf6HHDS4RolWfFhk4Rq9")]
  pub admin_password: String,
  /// Name of the site (can be changed later)
  #[doku(example = "My Lemmy Instance")]
  pub site_name: String,
  /// Email for the admin user (optional, can be omitted and set later through the website)
  #[doku(example = "user@example.com")]
  #[default(None)]
  pub admin_email: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, SmartDefault, Document)]
#[serde(deny_unknown_fields)]
pub struct PrometheusConfig {
  // Address that the Prometheus metrics will be served on.
  #[default(Some(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))))]
  #[doku(example = "127.0.0.1")]
  pub bind: Option<IpAddr>,
  // Port that the Prometheus metrics will be served on.
  #[default(Some(10002))]
  #[doku(example = "10002")]
  pub port: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize, Clone, SmartDefault, Document)]
pub struct PiNetworkConfig {
  // Allow non-pioneers register by email / username
  #[default(false)]
  pub pi_allow_all: bool,

  // Allow use Pi Browser to login, by pass registration process
  #[default(true)]
  pub pi_free_login: bool,

  #[default(Some("changeme".to_string()))]
  //#[default(None)]
  pub pi_seed: Option<String>,
  #[default(Some("changeme".to_string()))]
  pub pi_key: Option<String>,
  #[default(Some("https://api.minepi.com/v2".to_string()))]
  pub pi_api_host: Option<String>,
  #[default(true)]
  pub pi_testnet: bool,
  #[default(Some("https://api.testnet.minepi.com/".to_string()))]
  pub pi_horizon_host: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, SmartDefault, Document)]
pub struct Web3Config {
  // Allow non-pioneers register by email / username
  #[default(false)]
  pub enabled: bool,

  // Key schema
  #[default(0)]
  pub key_schema: i32,

  #[default(0)]
  pub chain_id: i32,
  #[default(None)]
  pub admin: Option<String>,
  #[default(None)]
  pub signer: Option<String>,
  #[default(None)]
  pub signer_key: Option<String>,
  #[default(None)]
  pub api_host: Option<String>,
}
