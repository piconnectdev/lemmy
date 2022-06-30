#[cfg(feature = "full")]
use crate::schema::secret;
use crate::newtypes::{SecretId};

#[derive(Clone)]
#[cfg_attr(feature = "full", derive(Queryable, Identifiable))]
#[cfg_attr(feature = "full", table_name = "secret")]
pub struct Secret {
  pub id: SecretId,
  pub jwt_secret: String,
}
