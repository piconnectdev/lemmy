use crate::{
  newtypes::{PersonId, PiPaymentId, PiUserId},
  source::pipayment::*,
  //traits::{Crud, ToSafe, },
  traits::Crud,
  utils::{get_conn, naive_now, DbPool},
};
use diesel::result::Error;
use diesel::{dsl::insert_into, ExpressionMethods, QueryDsl};
use diesel_async::RunQueryDsl;

/*
mod safe_type {
  use crate::{
    schema::pipayment::{
      a2u, amount, approved, asset, cancelled, comment, completed, created_at, direction, domain,
      extras, fee, finished, from_address, id, identifier, instance_id, memo, metadata, network,
      obj_cat, obj_id, person_id, pi_uid, pi_username, published, ref_id, stat, step, testnet,
      to_address, tx_id, tx_link, tx_verified, updated, user_cancelled, user_uid, verified,
    },
    source::pipayment::PiPayment,
    //traits::ToSafe,
  };

  type Columns = (
    id,
    //domain,
    //instance_id,
    person_id,
    obj_cat,
    obj_id,
    a2u,
    step,
    asset,
    fee,
    testnet,
    finished,
    published,
    updated,
    ref_id,
    comment,
    stat,
    //pi_uid,
    //pi_username,
    identifier,
    user_uid,
    amount,
    memo,
    from_address,
    to_address,
    direction,
    created_at,
    approved,
    verified,
    completed,
    cancelled,
    user_cancelled,
    tx_verified,
    tx_link,
    tx_id,
    network,
    metadata,
    extras,
  );

  impl ToSafe for PiPayment {
    type SafeColumns = Columns;
    fn safe_columns_tuple() -> Self::SafeColumns {
      (
        id,
        //domain,
        //instance_id,
        person_id,
        obj_cat,
        obj_id,
        a2u,
        step,
        asset,
        fee,
        testnet,
        finished,
        published,
        updated,
        ref_id,
        comment,
        stat,
        //pi_uid,
        //pi_username,
        identifier,
        user_uid,
        amount,
        memo,
        from_address,
        to_address,
        direction,
        created_at,
        approved,
        verified,
        completed,
        cancelled,
        user_cancelled,
        tx_verified,
        tx_link,
        tx_id,
        network,
        metadata,
        extras,
      )
    }
  }
}
*/

// impl PiPaymentSafe {
//   pub async fn find_by_person(pool: &DbPool, pid: &PersonId) -> Result<Vec<Self>, Error> {
//     use crate::schema::pipayment::dsl::*;
//     let conn = &mut get_conn(pool).await?;
//     pipayment
//       .filter(person_id.eq(pid))
//       .select(PiPayment::safe_columns_tuple())
//       .order_by(published.desc())
//       .limit(50)
//       .get_results::<Self>(conn)
//       .await
//   }
// }

#[async_trait]
impl Crud for PiPayment {
  type InsertForm = PiPaymentInsertForm;
  type UpdateForm = PiPaymentUpdateForm;
  type IdType = PiPaymentId;
  async fn read(pool: &DbPool, pid: PiPaymentId) -> Result<Self, Error> {
    use crate::schema::pipayment::dsl::*;
    let conn = &mut get_conn(pool).await?;
    pipayment.find(pid).first::<Self>(conn).await
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
    diesel::delete(pipayment.find(payment_id))
      .execute(conn)
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
  pub async fn find_withdraw_pending(pool: &DbPool, pid: &PersonId) -> Result<Vec<Self>, Error> {
    use crate::schema::pipayment::dsl::*;
    let conn = &mut get_conn(pool).await?;
    pipayment
      .filter(person_id.eq(pid))
      .filter(a2u.eq(1))
      .filter(finished.eq(false))
      .filter(step.eq(0))
      .get_results::<Self>(conn)
      .await
  }
  pub async fn find_pending(pool: &DbPool, pid: Option<PiPaymentId>) -> Result<Self, Error> {
    use crate::schema::pipayment::dsl::*;
    let conn = &mut get_conn(pool).await?;
    if pid.is_some() {
      pipayment
        .filter(id.eq(pid.unwrap()))
        .filter(a2u.eq(1))
        .filter(finished.eq(false))
        .first::<Self>(conn)
        .await
    } else {
      pipayment
        .filter(a2u.eq(1))
        .filter(finished.eq(false))
        .filter(step.eq(1))
        .first::<Self>(conn)
        .await
    }
  }

  pub async fn update_step(
    pool: &DbPool,
    payment_id: PiPaymentId,
    new_step: i32,
  ) -> Result<Self, Error> {
    use crate::schema::pipayment::dsl::*;
    let conn = &mut get_conn(pool).await?;
    diesel::update(pipayment.find(payment_id))
      .set((step.eq(new_step), updated.eq(naive_now())))
      .get_result::<Self>(conn)
      .await
  }

  pub async fn update_pending(
    pool: &DbPool,
    payment_id: PiPaymentId,
    new_payment: &PiPaymentUpdatePending,
  ) -> Result<Self, Error> {
    use crate::schema::pipayment::dsl::*;
    let conn = &mut get_conn(pool).await?;
    diesel::update(pipayment.find(payment_id))
      .set(new_payment)
      .get_result::<Self>(conn)
      .await
  }
}

#[cfg(test)]
mod tests {
  use lemmy_utils::settings::SETTINGS;
  use uuid::Uuid;

  use crate::{
    newtypes::PiUserId, source::pipayment::*, traits::Crud, utils::build_db_pool_for_tests,
    utils::naive_now,
  };
  use serial_test::serial;
  #[tokio::test]
  #[serial]
  async fn test_crud() {
    let settings = SETTINGS.to_owned();
    let pool = &build_db_pool_for_tests().await;
    let uid = Uuid::new_v4();

    let new_payment = PiPaymentInsertForm::builder()
      .domain(None)
      .instance_id(None)
      .person_id(None)
      .obj_cat(None)
      .obj_id(Some(uid))
      .a2u(0)
      .asset(None)
      .fee(0.00)
      .step(0)
      .ref_id(Some(uid))
      .testnet(settings.pinetwork.pi_testnet)
      .finished(false)
      .updated(None)
      .pi_uid(Some(PiUserId(uid.clone())))
      .pi_username("wepi".into())
      .comment(None)
      .identifier(Some(uid.hyphenated().to_string()))
      .user_uid(Some(uid.hyphenated().to_string()))
      .amount(0.000)
      .memo(None)
      .to_address(None)
      .from_address(None)
      .direction(None)
      .network(None)
      .created_at(Some(naive_now()))
      .approved(true)
      .verified(true)
      .completed(false)
      .cancelled(false)
      .user_cancelled(false)
      .tx_link(None)
      .tx_id(None)
      .tx_verified(false)
      .metadata(None)
      .extras(None)
      //.instance_id(inserted_instance.id)
      .build();

    let inserted_payment = PiPayment::create(pool, &new_payment).await.unwrap();

    let expected_payment = PiPayment::builder()
      .id(inserted_payment.id)
      .domain(None)
      .instance_id(None)
      .person_id(None)
      .obj_cat(None)
      .obj_id(Some(uid))
      .asset(None)
      .a2u(0)
      .fee(0.00)
      .step(0)
      .ref_id(Some(uid))
      .testnet(settings.pinetwork.pi_testnet)
      .published(inserted_payment.published.clone())
      .created_at(inserted_payment.created_at.clone())
      .finished(false)
      .updated(None)
      .pi_uid(Some(PiUserId(uid.clone())))
      .pi_username("wepi".into())
      .comment(None)
      .stat(None)
      .identifier(Some(uid.hyphenated().to_string()))
      .user_uid(Some(uid.hyphenated().to_string()))
      .amount(0.00)
      .memo(None)
      .to_address(None)
      .from_address(None)
      .direction(None)
      .network(None)
      .approved(true)
      .verified(true)
      .completed(false)
      .cancelled(false)
      .user_cancelled(false)
      .tx_link(None)
      .tx_id(None)
      .tx_verified(false)
      .metadata(None)
      .extras(None)
      .build();

    let read_payment = PiPayment::read(pool, inserted_payment.id).await.unwrap();
    let update_payment_form = PiPaymentUpdateForm::builder()
      //.amount(0.001)
      .approved(true)
      .verified(true)
      //.memo("wepi.social".into())
      .tx_id(None)
      .build();
    let updated_payment = PiPayment::update(pool, inserted_payment.id, &update_payment_form)
      .await
      .unwrap();
    let num_deleted = PiPayment::delete(pool, inserted_payment.id).await.unwrap();

    assert_eq!(expected_payment, read_payment);
    assert_eq!(expected_payment, inserted_payment);
    assert_eq!(expected_payment, updated_payment);
    assert_eq!(1, num_deleted);
  }
}
