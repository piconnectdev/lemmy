use crate::{
  newtypes::{PersonBalanceId, PersonId},
  schema::person_balance::{
    amount, asset, deposited, dsl::person_balance, pending, person_id, received, spent, updated,
    withdrawed,
  },
  source::person_balance::{PersonBalance, PersonBalanceInsertForm, PersonBalanceUpdateForm},
  traits::Crud,
  utils::{get_conn, naive_now, DbPool},
};

use diesel::{dsl::insert_into, result::Error, ExpressionMethods, QueryDsl};
use diesel_async::RunQueryDsl;

#[async_trait]
impl Crud for PersonBalance {
  type InsertForm = PersonBalanceInsertForm;
  type UpdateForm = PersonBalanceUpdateForm;
  type IdType = PersonBalanceId;

  async fn read(pool: &DbPool, person_balance_id: PersonBalanceId) -> Result<Self, Error> {
    let conn = &mut get_conn(pool).await?;
    person_balance
      .find(person_balance_id)
      .first::<Self>(conn)
      .await
  }

  async fn delete(pool: &DbPool, person_balance_id: PersonBalanceId) -> Result<usize, Error> {
    let conn = &mut get_conn(pool).await?;
    diesel::delete(person_balance.find(person_balance_id))
      .execute(conn)
      .await
  }

  async fn create(pool: &DbPool, form: &Self::InsertForm) -> Result<Self, Error> {
    let conn = &mut get_conn(pool).await?;
    // since the return here isnt utilized, we dont need to do an update
    // but get_result doesnt return the existing row here
    insert_into(person_balance)
      .values(form)
      //.on_conflict((recipient_id, comment_id))
      //.do_update()
      //.set(person_balance_form)
      .get_result::<Self>(conn)
      .await
  }

  async fn update(
    pool: &DbPool,
    person_balance_id: PersonBalanceId,
    form: &Self::UpdateForm,
  ) -> Result<Self, Error> {
    let conn = &mut get_conn(pool).await?;
    diesel::update(person_balance.find(person_balance_id))
      .set(form)
      .get_result::<Self>(conn)
      .await
  }
}

impl PersonBalance {
  pub async fn find_by_asset(
    pool: &DbPool,
    person_balance_id: PersonId,
    asset_name: &str,
  ) -> Result<Self, Error> {
    use crate::schema::person_balance::dsl::*;
    let conn = &mut get_conn(pool).await?;
    person_balance
      .filter(person_id.eq(person_balance_id))
      .filter(asset.eq(asset_name))
      .first::<Self>(conn)
      .await
  }

  pub async fn update_deposit(pool: &DbPool, pid: PersonId, amt: f64) -> Result<Self, Error> {
    let conn = &mut get_conn(pool).await?;
    diesel::update(person_balance)
      .filter(person_id.eq(pid))
      .filter(asset.eq("PI".to_string()))
      .set((deposited.eq(deposited + amt), amount.eq(amount + amt)))
      .get_result::<Self>(conn)
      .await
  }

  pub async fn update_received(pool: &DbPool, pid: PersonId, amt: f64) -> Result<Self, Error> {
    let conn = &mut get_conn(pool).await?;
    diesel::update(person_balance)
      .filter(person_id.eq(pid))
      .filter(asset.eq("PI".to_string()))
      .set((received.eq(received + amt), amount.eq(amount + amt)))
      .get_result::<Self>(conn)
      .await
  }

  pub async fn update_withdraw(
    pool: &DbPool,
    pid: PersonId,
    amt: f64,
    fee: f64,
  ) -> Result<Self, Error> {
    let conn = &mut get_conn(pool).await?;
    diesel::update(person_balance)
      .filter(person_id.eq(pid))
      .filter(asset.eq("PI".to_string()))
      .filter(amount.ge(amt + fee))
      .set((
        withdrawed.eq(withdrawed + amt + fee),
        amount.eq(amount - (amt + fee)),
        pending.eq(amt),
        updated.eq(naive_now()),
      ))
      .get_result::<Self>(conn)
      .await
  }

  pub async fn update_spent(
    pool: &DbPool,
    pid: PersonId,
    amt: f64,
    fee: f64,
  ) -> Result<Self, Error> {
    let conn = &mut get_conn(pool).await?;
    diesel::update(person_balance)
      .filter(person_id.eq(pid))
      .filter(asset.eq("PI".to_string()))
      .set((spent.eq(spent + amt + fee),))
      .get_result::<Self>(conn)
      .await
  }
}

#[cfg(test)]
mod tests {
  use crate::{
    source::{
      instance::Instance,
      person::{Person, PersonInsertForm},
      person_balance::{PersonBalance, PersonBalanceInsertForm, PersonBalanceUpdateForm},
    },
    traits::Crud,
    utils::build_db_pool_for_tests,
  };
  use serial_test::serial;

  #[tokio::test]
  #[serial]
  async fn test_crud() {
    let pool = &build_db_pool_for_tests().await;

    let inserted_instance = Instance::read_or_create(pool, "my_domain.tld".to_string())
      .await
      .unwrap();

    let new_person = PersonInsertForm::builder()
      .name("terrylake".into())
      .public_key("pubkey".to_string())
      .instance_id(inserted_instance.id)
      .build();

    let inserted_person = Person::create(pool, &new_person).await.unwrap();

    let person_balance_form = PersonBalanceInsertForm::builder()
      .person_id(inserted_person.id.clone())
      .asset(Some("PI".to_string()))
      .deposited(0.0)
      .received(0.0)
      .withdrawed(0.0)
      .spent(0.0)
      .amount(0.0)
      .pending(0.0)
      .build();

    let inserted_balance = PersonBalance::create(pool, &person_balance_form)
      .await
      .unwrap();

    let expected_balance = PersonBalance::builder()
      .id(inserted_balance.id.clone())
      .person_id(inserted_person.id.clone())
      .asset(Some("PI".to_string()))
      .published(inserted_balance.published.clone())
      .updated(None)
      .deposited(0.0)
      .received(0.0)
      .withdrawed(0.0)
      .spent(0.0)
      .amount(0.0)
      .pending(0.0)
      .extras(None)
      .build();

    let read_balance = PersonBalance::read(pool, inserted_balance.id)
      .await
      .unwrap();

    let person_balance_update_form = PersonBalanceUpdateForm::builder()
      .deposited(0.0)
      .received(0.0)
      .withdrawed(0.0)
      .spent(0.0)
      .amount(0.0)
      .pending(0.0)
      .build();

    let updated_balance =
      PersonBalance::update(pool, inserted_balance.id, &person_balance_update_form)
        .await
        .unwrap();
    Person::delete(pool, inserted_person.id).await.unwrap();
    PersonBalance::delete(pool, updated_balance.id)
      .await
      .unwrap();
    Instance::delete(pool, inserted_instance.id).await.unwrap();

    assert_eq!(expected_balance, read_balance);
    assert_eq!(expected_balance, inserted_balance);
    assert_eq!(expected_balance, updated_balance);
  }
}
