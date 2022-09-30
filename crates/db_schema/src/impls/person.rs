use crate::{
  newtypes::{DbUrl, PersonId},
  schema::person::dsl::*,
  source::person::{Person, PersonForm},
  traits::{ApubActor, Crud},
  utils::{functions::lower, naive_now},
};
use diesel::{
  dsl::*,
  result::Error,
  ExpressionMethods,
  PgConnection,
  QueryDsl,
  RunQueryDsl,
  TextExpressionMethods,
};
use sha2::{Digest, Sha256};

mod safe_type {
  use crate::{schema::person::columns::*, source::person::Person, traits::ToSafe};

  type Columns = (
    id,
    name,
    display_name,
    avatar,
    banned,
    published,
    updated,
    actor_id,
    bio,
    local,
    banner,
    deleted,
    inbox_url,
    shared_inbox_url,
    matrix_user_id,
    admin,
    bot_account,
    ban_expires,
    verified,
    pi_address,
    web3_address,
    sol_address,
    dap_address,
    cosmos_address,
    auth_sign,
    srv_sign,
    tx,
  );

  impl ToSafe for Person {
    type SafeColumns = Columns;
    fn safe_columns_tuple() -> Self::SafeColumns {
      (
        id,
        name,
        display_name,
        avatar,
        banned,
        published,
        updated,
        actor_id,
        bio,
        local,
        banner,
        deleted,
        inbox_url,
        shared_inbox_url,
        matrix_user_id,
        admin,
        bot_account,
        ban_expires,
        verified,
        pi_address,
        web3_address,
        sol_address,
        dap_address,
        cosmos_address,
        auth_sign,
        srv_sign,
        tx,
      )
    }
  }
}

impl Crud for Person {
  type Form = PersonForm;
  type IdType = PersonId;
  fn read(conn: &mut PgConnection, person_id: PersonId) -> Result<Self, Error> {
    person
      .filter(deleted.eq(false))
      .find(person_id)
      .first::<Self>(conn)
  }
  fn delete(conn: &mut PgConnection, person_id: PersonId) -> Result<usize, Error> {
    diesel::delete(person.find(person_id)).execute(conn)
  }
  fn create(conn: &mut PgConnection, form: &PersonForm) -> Result<Self, Error> {
    insert_into(person).values(form).get_result::<Self>(conn)
  }
  fn update(
    conn: &mut PgConnection,
    person_id: PersonId,
    form: &PersonForm,
  ) -> Result<Self, Error> {
    diesel::update(person.find(person_id))
      .set(form)
      .get_result::<Self>(conn)
  }
}

impl Person {
  pub fn ban_person(
    conn: &mut PgConnection,
    person_id: PersonId,
    ban: bool,
    expires: Option<chrono::NaiveDateTime>,
  ) -> Result<Self, Error> {
    diesel::update(person.find(person_id))
      .set((banned.eq(ban), ban_expires.eq(expires)))
      .get_result::<Self>(conn)
  }

  pub fn add_admin(
    conn: &mut PgConnection,
    person_id: PersonId,
    added: bool,
  ) -> Result<Self, Error> {
    diesel::update(person.find(person_id))
      .set(admin.eq(added))
      .get_result::<Self>(conn)
  }

  pub fn mark_as_updated(conn: &mut PgConnection, person_id: PersonId) -> Result<Person, Error> {
    diesel::update(person.find(person_id))
      .set((last_refreshed_at.eq(naive_now()),))
      .get_result::<Self>(conn)
  }

  pub fn delete_account(conn: &mut PgConnection, person_id: PersonId) -> Result<Person, Error> {
    use crate::schema::local_user;

    // Set the local user info to none
    diesel::update(local_user::table.filter(local_user::person_id.eq(person_id)))
      .set((
        local_user::email.eq::<Option<String>>(None),
        local_user::validator_time.eq(naive_now()),
      ))
      .execute(conn)?;

    diesel::update(person.find(person_id))
      .set((
        display_name.eq::<Option<String>>(None),
        avatar.eq::<Option<String>>(None),
        banner.eq::<Option<String>>(None),
        bio.eq::<Option<String>>(None),
        matrix_user_id.eq::<Option<String>>(None),
        deleted.eq(true),
        updated.eq(naive_now()),
      ))
      .get_result::<Self>(conn)
  }

  pub fn upsert(conn: &mut PgConnection, person_form: &PersonForm) -> Result<Person, Error> {
    insert_into(person)
      .values(person_form)
      .on_conflict(actor_id)
      .do_update()
      .set(person_form)
      .get_result::<Self>(conn)
  }

  pub fn update_deleted(
    conn: &mut PgConnection,
    person_id: PersonId,
    new_deleted: bool,
  ) -> Result<Person, Error> {
    use crate::schema::person::dsl::*;
    diesel::update(person.find(person_id))
      .set(deleted.eq(new_deleted))
      .get_result::<Self>(conn)
  }

  pub fn leave_admin(conn: &mut PgConnection, person_id: PersonId) -> Result<Self, Error> {
    diesel::update(person.find(person_id))
      .set(admin.eq(false))
      .get_result::<Self>(conn)
  }

  pub fn remove_avatar_and_banner(
    conn: &mut PgConnection,
    person_id: PersonId,
  ) -> Result<Self, Error> {
    diesel::update(person.find(person_id))
      .set((
        avatar.eq::<Option<String>>(None),
        banner.eq::<Option<String>>(None),
      ))
      .get_result::<Self>(conn)
  }

  pub fn find_by_name(conn: &mut PgConnection, from_name: &str) -> Result<Person, Error> {
    person

      .filter(deleted.eq(false))
      .filter(local.eq(true))
      .filter(name.eq(from_name))
      .first::<Person>(conn)
  }

  pub fn find_by_pi_name(conn: &mut PgConnection, from_name: &str) -> Result<Person, Error> {
    person
      .filter(deleted.eq(false))
      .filter(local.eq(true))
      .filter(extra_user_id.eq(from_name))
      .first::<Person>(conn)
  }
  
  pub fn find_by_web3_address(conn: &mut PgConnection, from_name: &str) -> Result<Person, Error> {
    person
      .filter(deleted.eq(false))
      .filter(local.eq(true))
      .filter(web3_address.eq(from_name))
      .first::<Person>(conn)
  }

  pub fn update_srv_sign(
    conn: &mut PgConnection,
    person_id: PersonId,
    sig: &str,
  ) -> Result<Self, Error> {
    use crate::schema::person::dsl::*;
    diesel::update(person.find(person_id))
      .set(srv_sign.eq(sig))
      .get_result::<Self>(conn)
  }

  pub fn sign_data(data: &Person) -> (Option<String>, Option<String>, Option<String>) {    
    let mut sha_meta = Sha256::new();
    let mut sha_content = Sha256::new();
    let mut sha256 = Sha256::new();

    sha_meta.update(format!("{}",data.id.clone().0.simple()));
    sha_meta.update(format!("{}",data.actor_id.clone().to_string()));
    sha_meta.update(format!("{}",data.published.clone().to_string()));
    let meta:  String = format!("{:x}", sha_meta.finalize());

    sha_content.update(data.name.clone().clone());
    let content:  String = format!("{:x}", sha_content.finalize());

    sha256.update(meta.clone());
    sha256.update(content.clone());
    let message: String = format!("{:x}", sha256.finalize());

    //let meta = lemmy_utils::utils::eth_sign_message(meta);
    let content = lemmy_utils::utils::eth_sign_message(content);
    let signature = lemmy_utils::utils::eth_sign_message(message);
    return (signature, Some(meta), content);
  }

}

pub fn is_banned(banned_: bool, expires: Option<chrono::NaiveDateTime>) -> bool {
  if let Some(expires) = expires {
    banned_ && expires.gt(&naive_now())
  } else {
    banned_
  }
}

impl ApubActor for Person {
  fn read_from_apub_id(conn: &mut PgConnection, object_id: &DbUrl) -> Result<Option<Self>, Error> {
    use crate::schema::person::dsl::*;
    Ok(
      person
        .filter(deleted.eq(false))
        .filter(actor_id.eq(object_id))
        .first::<Person>(conn)
        .ok()
        .map(Into::into),
    )
  }

  fn read_from_name(
    conn: &mut PgConnection,
    from_name: &str,
    include_deleted: bool,
  ) -> Result<Person, Error> {
    let mut q = person
      .into_boxed()
      .filter(local.eq(true))
      .filter(lower(name).eq(lower(from_name)));
    if !include_deleted {
      q = q.filter(deleted.eq(false))
    }
    q.first::<Self>(conn)
  }

  fn read_from_name_and_domain(
    conn: &mut PgConnection,
    person_name: &str,
    protocol_domain: &str,
  ) -> Result<Person, Error> {
    use crate::schema::person::dsl::*;
    person
      .filter(lower(name).eq(lower(person_name)))
      .filter(actor_id.like(format!("{}%", protocol_domain)))
      .first::<Self>(conn)
  }
}

#[cfg(test)]
mod tests {
  use lemmy_utils::utils::eth_sign_message;
  use sha2::{Digest, Sha256};
  use uuid::Uuid;
use crate::{source::person::*, traits::Crud, utils::establish_unpooled_connection};

  #[test]
  fn test_crud() {
    let conn = &mut establish_unpooled_connection();

    let new_person = PersonForm {
      name: "holly".into(),
      public_key: Some("nada".to_owned()),
      ..PersonForm::default()
    };

    let inserted_person = Person::create(conn, &new_person).unwrap();

    let mut sha256 = Sha256::new();
    sha256.update(format!("{}",inserted_person.id.0.simple()));
    sha256.update(inserted_person.name.clone());
    let message: String = format!("{:x}", sha256.finalize());
    let signature = eth_sign_message(message);
    Person::update_srv_sign(conn, inserted_person.id, signature.clone().unwrap_or_default().as_str()).unwrap();

    let expected_person = Person {
      id: inserted_person.id,
      name: "holly".into(),
      display_name: None,
      avatar: None,
      banner: None,
      banned: false,
      deleted: false,
      published: inserted_person.published,
      updated: None,
      actor_id: inserted_person.actor_id.to_owned(),
      bio: None,
      local: true,
      bot_account: false,
      admin: false,
      private_key: None,
      public_key: "nada".to_owned(),
      last_refreshed_at: inserted_person.published,
      inbox_url: inserted_person.inbox_url.to_owned(),
      shared_inbox_url: None,
      matrix_user_id: None,
      ban_expires: None,
      extra_user_id: None,
      verified: false,
      private_seeds: None,
      pi_address: None,
      web3_address: None,
      sol_address: None,
      dap_address: None,
      cosmos_address: None,
      auth_sign: None, 
      srv_sign: signature,
      tx : None,
    };

    
    let read_person = Person::read(conn, inserted_person.id).unwrap();
    let updated_person = Person::update(conn, inserted_person.id, &new_person).unwrap();
    let num_deleted = Person::delete(conn, inserted_person.id).unwrap();

    assert_eq!(expected_person, read_person);
    assert_eq!(expected_person, inserted_person);
    assert_eq!(expected_person, updated_person);
    assert_eq!(1, num_deleted);
  }
}
