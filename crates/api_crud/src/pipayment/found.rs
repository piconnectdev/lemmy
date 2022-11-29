use crate::pipayment::client::*;
use crate::PerformCrud;
use actix_web::web::Data;
use lemmy_api_common::{pipayment::*, };
use lemmy_db_schema::{
  //impls::pipayment::PiPaymentModerator, 
  newtypes::PiPaymentId, source::pipayment::*, traits::Crud,
  utils::naive_now,
};

use lemmy_utils::{error::LemmyError, settings::SETTINGS, ConnectionId};
use lemmy_websocket::LemmyContext;
use sha2::{Digest, Sha256};
use uuid::Uuid;

#[async_trait::async_trait(?Send)]
impl PerformCrud for PiPaymentFound {
  type Response = PiPaymentFoundResponse;

  async fn perform(
    &self,
    context: &Data<LemmyContext>,
    _websocket_id: Option<ConnectionId>,
  ) -> Result<PiPaymentFoundResponse, LemmyError> {
    let settings = SETTINGS.to_owned();
    let data: &PiPaymentFound = self;

    let mut sha256 = Sha256::new();
    sha256.update(settings.pi_seed());
    sha256.update(data.pi_username.to_owned());
    let _pi_username: String = format!("{:X}", sha256.finalize());
    let _pi_username2 = _pi_username.clone();

    let _payment_id = data.paymentid.to_owned();
    let _payment_id2 = _payment_id.clone();
    let _pi_username = data.pi_username.to_owned();
    let _pi_uid = data.pi_uid.clone();

    let mut exist = false;
    let mut payment_id: PiPaymentId;
    let payment;
    let mut comment: String = "".to_string();
    let mut memo: String;
    let mut ref_id: Option<Uuid> = None;
    let mut updated: Option<chrono::NaiveDateTime> = None;

    let mut dto_source: i32 = 0;

    let mut approved = false;
    let mut completed = false;
    let mut finished = false;
    let mut cancelled = false;
    let mut txid: String;
    let txlink: String;
    let mut dto: Option<PiPaymentDto> = None;

    let mut _payment = match PiPayment::find_by_pipayment_id(context.pool(), &_payment_id).await
    {
      Ok(c) => {
        exist = true;
        payment_id = c.id;
        comment = c.comment.clone().unwrap();
        ref_id = c.ref_id;

        approved = c.approved;
        completed = c.completed;
        cancelled = c.cancelled;
        finished = c.finished;
        txid = c.tx_id.clone();
        txlink = c.tx_link.clone();
        memo = c.memo.clone();
        Some(c)
      }
      Err(_e) => None,
    };

    if _payment.is_some() {
      exist = true;
      updated = Some(naive_now());
      //txid = payment.tx_id.clone();
    } else {
      exist = false;
    }

    let dto_read = match pi_payment(context.client(), &data.paymentid.clone()).await {
      Ok(c) => {
        approved = c.status.developer_approved;
        completed = c.status.developer_completed;
        cancelled = c.status.cancelled;
        memo = c.memo.clone();
        dto_source = 1;
        c
      }
      Err(_e) => {
        // Pi Server error
        let err_type = format!(
          "Pi Server: error while check payment: user {}, paymentid {} error: {}",
          &data.pi_username,
          &data.paymentid,
          _e.to_string()
        );
        return Err(LemmyError::from_message(&err_type));
      }
    };

    if cancelled {
      let err_type = format!(
        "Pi Server: payment cancelled: user {}, paymentid {}",
        &data.pi_username, &data.paymentid
      );
      return Err(LemmyError::from_message(&err_type));
    }

    if !approved {
      dto = match pi_approve(context.client(), &data.paymentid.clone()).await {
        Ok(c) => {
          dto_source = 2;
          Some(c)
        }
        Err(_e) => {
          // Pi Server error
          let err_type = format!(
            "Pi Server: Error while approve: user {}, paymentid {} error: {}",
            &data.pi_username,
            &data.paymentid,
            _e.to_string()
          );
          //let err_type = _e.to_string();
          return Err(LemmyError::from_message(&err_type));
        }
      };
    } else {
      if !completed {
        let tx = dto_read.transaction.clone();
        match tx {
          Some(_tx) => {
            txid = _tx.txid;
            dto = match pi_complete(context.client(), &data.paymentid.clone(), &txid.clone()).await
            {
              Ok(c) => {
                dto_source = 3;
                Some(c)
              }
              Err(_e) => {
                // Pi Server error
                let err_type = format!(
                  "Pi Server: Error while completed: user {}, paymentid {} error: {}",
                  &data.pi_username,
                  &data.paymentid,
                  _e.to_string()
                );
                return Err(LemmyError::from_message(&err_type));
              }
            };
          }
          None => {
            let err_type = format!(
              "Pi Server: Error while completed, no transaction: user {}, paymentid {}",
              &data.pi_username, &data.paymentid
            );
            return Err(LemmyError::from_message(&err_type));
          }
        };
      };
      finished = true;
    }

    let mut _payment_dto = PiPaymentDto {
      ..PiPaymentDto::default()
    };

    if dto.is_some() {
      _payment_dto = dto.unwrap();
    } else {
      _payment_dto = dto_read;
    }

    _payment_dto.status.developer_approved = true;

    let create_at = match chrono::NaiveDateTime::parse_from_str(
      &_payment_dto.created_at,
      "%Y-%m-%dT%H:%M:%S%.f%Z",
    ) {
      Ok(dt) => Some(dt),
      Err(_e) => {
        let err_type = format!(
          "Pi Server: get payment datetime error: user {}, paymentid {} {} {}",
          &data.pi_username,
          &data.paymentid,
          _payment_dto.created_at,
          _e.to_string()
        );
        //return Err(LemmyError::from_message(&err_type));
        None
      }
    };

    let _comment = format!(
      "UpdatePaymentFound;dto_source:{};pi_pid:{};memo:{};{}",
      dto_source,
      data.paymentid.clone(),
      memo.clone(),
      comment
    );


    if !exist {
      let mut payment_form = PiPaymentInsertForm::builder()
      .person_id( None)
      .ref_id( ref_id.clone())
      .testnet( settings.pinetwork.pi_testnet)
      .finished( finished)
      .updated( updated)
      .pi_uid( data.pi_uid)         //data.pi_uid
      .pi_username( "".to_string()) //data.pi_username.clone(), Hide user name
      .comment( Some(_comment))     //"".to_string(),

      .identifier( data.paymentid.clone())
      .user_uid( _payment_dto.user_uid.clone()) //"".to_string(), //_payment_dto.user_uid,
      .amount( _payment_dto.amount)
      .memo( memo)
      .to_address( _payment_dto.to_address)
      .created_at( create_at)
      .approved( _payment_dto.status.developer_approved)
      .verified( _payment_dto.status.transaction_verified)
      .completed( _payment_dto.status.developer_completed)
      .cancelled( _payment_dto.status.cancelled)
      .user_cancelled( _payment_dto.status.user_cancelled)
      .tx_link( "".to_string())
      .tx_id( "".to_string())
      .tx_verified( false)
      .metadata( _payment_dto.metadata)
      .extras( None)
      //tx_id:  _payment_dto.transaction.map(|tx| tx.txid),
      //..PiPaymentForm::default()
      .build();

      match _payment_dto.transaction {
        Some(tx) => {
          payment_form.tx_link = tx._link;
          payment_form.tx_verified = tx.verified;
          payment_form.tx_id = tx.txid;
        }
        None => {}
      }
      _payment = match PiPayment::create(context.pool(), &payment_form).await
      {
        Ok(payment) => {
          payment_id = payment.id;
          Some(payment)
        }
        Err(_e) => {
          // let err_type = if e.to_string() == "value too long for type character varying(200)" {
          //   "post_title_too_long"
          // } else {
          //   "couldnt_create_post"
          // };
          let err_type = format!(
            "Error insert payment: user {}, paymentid {} error: {}",
            &data.pi_username,
            &data.paymentid,
            _e.to_string()
          );

          return Err(LemmyError::from_message(&err_type));
        }
      };
    } else {
      payment = _payment.unwrap();
      payment_id = payment.id;
      let mut payment_form = PiPaymentUpdateForm::builder()
              .build();
      _payment = match PiPayment::update(context.pool(), payment_id, &payment_form).await
      {
        Ok(payment) => Some(payment),
        Err(_e) => {
          // let err_type = if e.to_string() == "value too long for type character varying(200)" {
          //   "post_title_too_long"
          // } else {
          //   "couldnt_create_post"
          // };
          let err_type = format!(
            "Error update payment: user {}, paymentid {} error: {}",
            &data.pi_username,
            &data.paymentid,
            _e.to_string()
          );
          return Err(LemmyError::from_message(&err_type));
        }
      };
    }
    let payment = _payment.unwrap();
    Ok(PiPaymentFoundResponse {
      id: payment.id,
      paymentid: payment.identifier,
    })
  }
}
