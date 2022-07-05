use diesel::{dsl::*, result::Error, *};
use crate::{
  source::pipayment::*, 
  newtypes::{PaymentId, PiUserId}, 
  traits::{Crud, },
};

impl Crud for PiPayment {
  type Form = PiPaymentForm;
  type IdType = PaymentId;
  fn read(conn: &PgConnection, pipayment_id: PaymentId) -> Result<Self, Error> {
    use crate::schema::pipayment::dsl::*;
    pipayment.find(pipayment_id).first::<Self>(conn)
  }

  fn create(conn: &PgConnection, new_payment: &PiPaymentForm) -> Result<Self, Error> {
    use crate::schema::pipayment::dsl::*;
    insert_into(pipayment)
      .values(new_payment)
      .get_result::<Self>(conn)
  }

  fn update(
    conn: &PgConnection,
    payment_id: PaymentId,
    new_payment: &PiPaymentForm,
  ) -> Result<Self, Error> {
    use crate::schema::pipayment::dsl::*;
    diesel::update(pipayment.find(payment_id))
      .set(new_payment)
      .get_result::<Self>(conn)
  }

  fn delete(conn: &PgConnection, payment_id: PaymentId) -> Result<usize, Error> {
    use crate::schema::pipayment::dsl::*;
    diesel::delete(pipayment.find(payment_id)).execute(conn)
  }
}

pub trait PiPayment_ {
  fn find_by_pipayment_id(conn: &PgConnection, payment_id: &str) -> Result<PiPayment, Error>;
  fn find_by_pi_uid(conn: &PgConnection, pi_uid: &PiUserId) -> Result<PiPayment, Error>;
}

impl PiPayment_ for PiPayment { 
  fn find_by_pipayment_id(conn: &PgConnection, payment_id: &str) -> Result<Self, Error> {
    use crate::schema::pipayment::dsl::*;
    pipayment
      .filter(identifier.eq(payment_id))
      .first::<Self>(conn)
  }

  fn find_by_pi_uid(conn: &PgConnection, uid: &PiUserId) -> Result<Self, Error> {
    use crate::schema::pipayment::dsl::*;
    pipayment
      .filter(pi_uid.eq(uid))
      .first::<Self>(conn)
  }

}


#[cfg(test)]
mod tests {
use lemmy_utils::settings::SETTINGS;
use uuid::Uuid;

use crate::{utils::{establish_unpooled_connection, naive_now}, source::pipayment::*, newtypes::PiUserId, traits::Crud};

  #[test]
  fn test_crud() {
    let settings = SETTINGS.to_owned();
    let conn = establish_unpooled_connection();
    let uid = Uuid::new_v4();
    let new_payment = PiPaymentForm {
      person_id: None,
      ref_id: Some(uid),
      testnet: settings.pinetwork.pi_testnet,

      finished: false,
      updated: None,
      pi_uid: Some(PiUserId(uid.clone())),
      pi_username: "wepi".into(),
      comment: Some("wepi.social".into()),

      identifier: uid.to_hyphenated().to_string(),
      user_uid: uid.to_hyphenated().to_string(),
      amount: 0.001,
      memo: "wepi.social".into(),
      to_address: "".into(),
      created_at: Some(naive_now()),
      approved: true,
      verified: true,
      completed: false,
      cancelled: false,
      user_cancelled: false,
      tx_link: "".into(),
      tx_id: "".into(),
      tx_verified: false,
      metadata: None,
      extras: None,
      //..PiPaymentForm::default()
    };

    let inserted_payment = PiPayment::create(&conn, &new_payment).unwrap();

    let expected_payment = PiPayment {
      id: inserted_payment.id,
      person_id: None,
      ref_id: Some(uid),
      testnet: settings.pinetwork.pi_testnet,
      published: inserted_payment.published.clone(),
      finished: false,
      updated: None,
      pi_uid: Some(PiUserId(uid)),
      pi_username: "wepi".into(),
      comment: Some("wepi.social".into()),

      identifier: uid.to_hyphenated().to_string(),
      user_uid: uid.to_hyphenated().to_string(),
      amount: 0.001,
      memo: "wepi.social".into(),
      to_address: "".into(),
      created_at: inserted_payment.created_at.clone(),
      approved: true,
      verified: true,
      completed: false,
      cancelled: false,
      user_cancelled: false,
      tx_link: "".into(),
      tx_id: "".into(),
      tx_verified: false,
      metadata: None,
      extras: None,      
    };


    let read_payment = PiPayment::read(&conn, inserted_payment.id).unwrap();
    let updated_payment = PiPayment::update(&conn, inserted_payment.id, &new_payment).unwrap();
    let num_deleted = PiPayment::delete(&conn, inserted_payment.id).unwrap();

    assert_eq!(expected_payment, read_payment);
    assert_eq!(expected_payment, inserted_payment);
    assert_eq!(expected_payment, updated_payment);
    assert_eq!(1, num_deleted);
  }
}
