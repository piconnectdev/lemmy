use crate::Crud;
use diesel::{dsl::*, result::Error, *};
use lemmy_db_schema::{naive_now, source::pipayment::*, PaymentId, PersonId, PiPaymentId, PiUserId};
use uuid::Uuid;

impl Crud<PaymentForm, Uuid> for Payment {
  fn read(conn: &PgConnection, _payment_id: PaymentId) -> Result<Self, Error> {
    use lemmy_db_schema::schema::payment::dsl::*;
    payment.find(PaymentId).first::<Self>(conn)
  }

  fn create(conn: &PgConnection, new_payment: &PaymentForm) -> Result<Self, Error> {
    use lemmy_db_schema::schema::payment::dsl::*;
    insert_into(payment)
      .values(new_payment)
      .get_result::<Self>(conn)
  }

  fn update(
    conn: &PgConnection,
    payment_id: PaymentId,
    new_payment: &PaymentForm,
  ) -> Result<Self, Error> {
    use lemmy_db_schema::schema::payment::dsl::*;
    diesel::update(payment.find(payment_id))
      .set(new_payment)
      .get_result::<Self>(conn)
  }

  fn delete(conn: &PgConnection, payment_id: PaymentId) -> Result<usize, Error> {
    use lemmy_db_schema::schema::payment::dsl::*;
    diesel::delete(payment.find(payment_id)).execute(conn)
  }
}

pub trait Payment_ {
  //fn transfer(conn: &PgConnection, new_creator_id: PersonId) -> Result<Site, Error>;
  //fn read_simple(conn: &PgConnection) -> Result<Payment, Error>;
  fn find_by_pipayment_id(conn: &PgConnection, payment_id: &PiPaymentId) -> Result<Self, Error>;
}

impl Payment_ for Payment { 
  fn find_by_pipayment_id(conn: &PgConnection, payment_id: &PiPaymentId) -> Result<Self, Error> {
    use lemmy_db_schema::schema::payment::dsl::*;
    payment
      .filter(payment::pi_payment_id == _payment_id)
      .first::<Self>(conn)
  }
}
