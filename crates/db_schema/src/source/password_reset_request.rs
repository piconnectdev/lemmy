use crate::newtypes::{LocalUserId, PasswordResetId};

#[cfg(feature = "full")]
use crate::schema::password_reset_request;

#[derive(PartialEq, Eq, Debug)]
#[cfg_attr(feature = "full", derive(Queryable, Identifiable))]
#[cfg_attr(feature = "full", diesel(table_name = password_reset_request))]
pub struct PasswordResetRequest {
  pub id: PasswordResetId,
  pub token_encrypted: String,
  pub published: chrono::NaiveDateTime,
  pub local_user_id: LocalUserId,
}

#[cfg_attr(feature = "full", derive(Insertable, AsChangeset))]
#[cfg_attr(feature = "full", diesel(table_name = password_reset_request))]
pub struct PasswordResetRequestForm {
  pub local_user_id: LocalUserId,
  pub token_encrypted: String,
}
