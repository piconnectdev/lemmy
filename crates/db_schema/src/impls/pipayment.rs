use diesel::{result::Error,};
use crate::{
  source::pipayment::*, 
  newtypes::{PiPaymentId, PiUserId, PersonId}, 
  traits::{Crud, },
  utils::{get_conn, DbPool, }
};
use diesel::{dsl::insert_into, ExpressionMethods, QueryDsl, };
use diesel_async::RunQueryDsl;

#[async_trait]
impl Crud for PiPayment {
  type InsertForm = PiPaymentInsertForm;
  type UpdateForm = PiPaymentUpdateForm;
  type IdType = PiPaymentId;
  async fn read(pool: &DbPool, pid: PiPaymentId) -> Result<Self, Error> {
    use crate::schema::pipayment::dsl::*;
    let conn = &mut get_conn(pool).await?;
    pipayment.find(pid).first::<Self>(conn)
    .await
  }

  async fn create(pool: &DbPool, new_payment: &Self::InsertForm) -> Result<Self, Error> {
    use crate::schema::pipayment::dsl::*;
    let conn = &mut get_conn(pool).await?;
    insert_into(pipayment)
      .values(new_payment)
      .get_result::<Self>(conn)
      .await
  }

  async fn update(
    pool: &DbPool, 
    payment_id: PiPaymentId,
    new_payment: &Self::UpdateForm,
  ) -> Result<Self, Error> {
    use crate::schema::pipayment::dsl::*;
    let conn = &mut get_conn(pool).await?;
    diesel::update(pipayment.find(payment_id))
      .set(new_payment)
      .get_result::<Self>(conn)
      .await
  }

  async fn delete(pool: &DbPool, payment_id: PiPaymentId) -> Result<usize, Error> {
    use crate::schema::pipayment::dsl::*;
    let conn = &mut get_conn(pool).await?;
    diesel::delete(pipayment.find(payment_id)).execute(conn)
    .await
  }
}

// #[async_trait]
// pub trait PiPaymentModerator {
//   async fn find_by_pipayment_id(pool: &DbPool, payment_id: &str) -> Result<PiPayment, Error>;
//   async fn find_by_pi_uid(pool: &DbPool, pi_uid: &PiUserId) -> Result<Vec<PiPayment>, Error>;
//   async fn find_by_person(pool: &DbPool, person_id: &PersonId) -> Result<Vec<PiPayment>, Error>;
// }

//#[async_trait]
impl PiPayment { 
  pub async fn find_by_pipayment_id(pool: &DbPool, payment_id: &str) -> Result<Self, Error> {
    use crate::schema::pipayment::dsl::*;
    let conn = &mut get_conn(pool).await?;
    pipayment
      .filter(identifier.eq(payment_id))
      .first::<Self>(conn)
      .await
  }

  pub async fn find_by_pi_uid(pool: &DbPool, uid: &PiUserId) -> Result<Vec<Self>, Error> {
    use crate::schema::pipayment::dsl::*;
    let conn = &mut get_conn(pool).await?;
    pipayment
      .filter(pi_uid.eq(uid))
      .get_results::<Self>(conn)
      .await
  }

  pub async fn find_by_person(pool: &DbPool, pid: &PersonId) -> Result<Vec<Self>, Error> {
    use crate::schema::pipayment::dsl::*;
    let conn = &mut get_conn(pool).await?;
    pipayment
      .filter(person_id.eq(pid))
      .get_results::<Self>(conn)
      .await
  }
}


#[cfg(test)]
mod tests {
use lemmy_utils::settings::SETTINGS;
use uuid::Uuid;

use crate::{
    utils::{naive_now}, 
    utils::build_db_pool_for_tests,
  source::pipayment::*, newtypes::PiUserId, traits::Crud
};
use serial_test::serial;
#[tokio::test]
#[serial]
  async fn test_crud() {
    let settings = SETTINGS.to_owned();
    let pool = &build_db_pool_for_tests().await;
    let uid = Uuid::new_v4();

    let new_payment = PiPaymentInsertForm::builder()
      .domain(Some("wepi.social".into()))
      .instance_id(None)
      .person_id(None)
      .object_cat(Some("wepi".into()))
      .other_id(Some(uid))
      .object_id(Some(uid))
      .testnet(settings.pinetwork.pi_testnet)
      .finished(false)
      .updated(None)
      .pi_uid(Some(PiUserId(uid.clone())))
      .pi_username("wepi".into())
      .notes(Some("wepi.social".into()))
      .identifier(uid.hyphenated().to_string())
      .user_uid(uid.hyphenated().to_string())
      .amount(0.001)
      .memo("wepi.social".into())
      .to_address( "".into())
      .created_at(Some(naive_now()))
      .approved(true)
      .verified(true)
      .completed(false)
      .cancelled(false)
      .user_cancelled(false)
      .tx_link("".into())
      .tx_id( "".into())
      .tx_verified( false)
      .metadata(None)
      .extras(None)
      //.instance_id(inserted_instance.id)
      .build();
    

    let inserted_payment = PiPayment::create(pool, &new_payment).await.unwrap();

    let expected_payment = PiPayment::builder()
      .id(inserted_payment.id)
      .domain(Some("wepi.social".into()))
      .instance_id(None)
      .person_id(None)
      .other_id(Some(uid))
      .object_cat(Some("wepi".into()))
      .object_id(Some(uid))
      .testnet(settings.pinetwork.pi_testnet)
      .published(inserted_payment.published.clone())
      .created_at( inserted_payment.created_at.clone())
      .finished(false)
      .updated(None)
      .pi_uid(Some(PiUserId(uid.clone())))
      .pi_username("wepi".into())
      .notes(Some("wepi.social".into()))
      .identifier(uid.hyphenated().to_string())
      .user_uid(uid.hyphenated().to_string())
      .amount(0.001)
      .memo("wepi.social".into())
      .to_address( "".into())
      .approved(true)
      .verified(true)
      .completed(false)
      .cancelled(false)
      .user_cancelled(false)
      .tx_link("".into())
      .tx_id("".into())
      .tx_verified( false)
      .metadata(None)
      .extras(None)
      .build();

    let read_payment = PiPayment::read(pool, inserted_payment.id).await.unwrap();
    let update_payment_form = PiPaymentUpdateForm::builder()
        //.amount(0.001)
        .approved(true)
        .verified(true)
        //.memo("wepi.social".into())
        .tx_id("".into())
        .build();
    let updated_payment = PiPayment::update(pool, inserted_payment.id, &update_payment_form).await.unwrap();
    let num_deleted = PiPayment::delete(pool, inserted_payment.id).await.unwrap();

    assert_eq!(expected_payment, read_payment);
    assert_eq!(expected_payment, inserted_payment);
    assert_eq!(expected_payment, updated_payment);
    assert_eq!(1, num_deleted);
  }
}
