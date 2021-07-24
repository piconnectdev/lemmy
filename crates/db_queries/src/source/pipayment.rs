use crate::Crud;
use diesel::{dsl::*, result::Error, sql_types::*, *};
use lemmy_db_schema::{
  naive_now, 
  source::pipayment::*, 
  PersonId, 
  PaymentId, 
  PiPaymentId, 
  PiUserId  
};

use uuid::Uuid;

impl Crud<PiPaymentForm, PaymentId> for PiPayment {
  fn read(conn: &PgConnection, pipayment_id: PaymentId) -> Result<Self, Error> {
    use lemmy_db_schema::schema::pipayment::dsl::*;
    pipayment.find(pipayment_id).first::<Self>(conn)
  }

  fn create(conn: &PgConnection, new_payment: &PiPaymentForm) -> Result<Self, Error> {
    use lemmy_db_schema::schema::pipayment::dsl::*;
    insert_into(pipayment)
      .values(new_payment)
      .get_result::<Self>(conn)
  }

  fn update(
    conn: &PgConnection,
    payment_id: PaymentId,
    new_payment: &PiPaymentForm,
  ) -> Result<Self, Error> {
    use lemmy_db_schema::schema::pipayment::dsl::*;
    diesel::update(pipayment.find(payment_id))
      .set(new_payment)
      .get_result::<Self>(conn)
  }

  fn delete(conn: &PgConnection, payment_id: PaymentId) -> Result<usize, Error> {
    use lemmy_db_schema::schema::pipayment::dsl::*;
    diesel::delete(pipayment.find(payment_id)).execute(conn)
  }
}

pub trait PiPayment_ {
  fn find_by_pipayment_id(conn: &PgConnection, payment_id: PiPaymentId) -> Result<PiPayment, Error>;
}

impl PiPayment_ for PiPayment { 
  fn find_by_pipayment_id(conn: &PgConnection, payment_id: PiPaymentId) -> Result<Self, Error> {
    use lemmy_db_schema::schema::pipayment::dsl::*;
    pipayment
      .filter(pi_payment_id.eq(payment_id))
      .first::<Self>(conn)
  }
}
