use crate::newtypes::{LocalUserId, PersonId, RegistrationApplicationId};
#[cfg(feature = "full")]
use crate::schema::registration_application;
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "full", derive(Queryable, Identifiable))]
#[cfg_attr(feature = "full", diesel(table_name = registration_application))]
pub struct RegistrationApplication {
  pub id: RegistrationApplicationId,
  pub local_user_id: LocalUserId,
  pub answer: String,
  pub admin_id: Option<PersonId>,
  pub deny_reason: Option<String>,
  pub published: chrono::NaiveDateTime,
}

#[derive(Default)]
//#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "full", derive(Insertable))]
#[cfg_attr(feature = "full", diesel(table_name = registration_application))]
pub struct RegistrationApplicationInsertForm {
  pub local_user_id: LocalUserId,
  pub answer: String,
}

#[cfg_attr(feature = "full", derive(AsChangeset))]
#[cfg_attr(feature = "full", diesel(table_name = registration_application))]
pub struct RegistrationApplicationUpdateForm {
  pub admin_id: Option<Option<PersonId>>,
  pub deny_reason: Option<Option<String>>,
}
