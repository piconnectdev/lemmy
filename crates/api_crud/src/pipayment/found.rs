use crate::pipayment::client::*;
use crate::PerformCrud;
use actix_web::web::Data;
use lemmy_api_common::{pipayment::*, };
use lemmy_db_schema::{
  newtypes::{PiPaymentId, PersonId}, source::{pipayment::*, person::*}, traits::Crud,
  utils::naive_now,
};

use lemmy_utils::{error::LemmyError, settings::SETTINGS, ConnectionId};
use lemmy_api_common::{context::LemmyContext};
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

    if data.pi_token.is_none() {
      return Err(LemmyError::from_message("Pi token is missing!"));
    }
    
    let _pi_token = data.pi_token.clone().unwrap();
    let mut _pi_username = data.pi_username.to_owned();
    let mut _pi_uid = data.pi_uid.clone();

    let _payment_id = data.paymentid.clone();

    // First, valid user token
    let user_dto = match pi_me(context, &_pi_token.clone()).await {
      Ok(dto) => {
        _pi_username = dto.username.clone();
        _pi_uid = Some(dto.uid.clone());
        Some(dto)
      }
      Err(_e) => {
        // Pi Server error
        let err_type = format!(
          "Pi Network Server Error: User not found: {}, error: {}",
          &data.pi_username,
          _e.to_string()
        );
        return Err(LemmyError::from_message(&err_type));
      }
    };

    //let _tx = Some(data.txid.clone());
    let approve = PiApprove {
      domain: data.domain.clone(),
      pi_token: data.pi_token.clone(),
      pi_username: _pi_username.clone(),
      pi_uid: _pi_uid.clone(),
      paymentid: data.paymentid.clone(),
      object_id: None,
      comment: Some("PiPaymentFound".to_string()),
      auth: data.auth.clone(),
    };

    let _payment = match pi_payment_update(context, &approve, None).await {
      Ok(c) => c,
      Err(e) => {
        let err_type = e.to_string();
        return Err(LemmyError::from_message(&err_type));
      }
    };

    let payment = _payment.clone();
    return Ok(PiPaymentFoundResponse {
      id: payment.id,
      paymentid: payment.identifier,
    });

    //_pi_username = hide_username(&_pi_username.clone());
    
    let mut exist = false;
    let mut payment_id: PiPaymentId;
    let payment;
    let mut notes: String = "".to_string();
    let mut memo: String;
    let mut object_id: Option<Uuid> = None;
    let mut updated: Option<chrono::NaiveDateTime> = None;

    let mut dto_source: i32 = 0;

    let mut approved = false;
    let mut completed = false;
    let mut finished = false;
    let mut cancelled = false;
    let mut usercancelled = false;
    let mut txid: String = "".to_string();
    let mut txlink: String = "".to_string();
    let mut dto: Option<PiPaymentDto> = None;
    
    let mut person_id: Option<PersonId> = None;
    let person = match Person::find_by_extra_name(context.pool(), &_pi_username.clone()).await
    {
      Ok(c) => {
        person_id = Some(c.id.clone());
        Some(c)
      },
      Err(_e) => None
    };

    let mut _payment = match PiPayment::find_by_pipayment_id(context.pool(), &_payment_id).await
    {
      Ok(c) => {
        exist = true;
        payment_id = c.id;
        notes = c.comment.clone().unwrap_or_default();
        object_id = c.ref_id;

        approved = c.approved;
        completed = c.completed;
        cancelled = c.cancelled;
        finished = c.finished;
        usercancelled = c.user_cancelled;
        txid = c.tx_id.clone();
        txlink = c.tx_link.clone();
        memo = c.memo.clone();
        Some(c)
      }
      Err(_e) => None,
    };

    if _payment.is_some() {
      updated = Some(naive_now());
    } else {
      exist = false;
    }

    let dto_read = match pi_payment(context.client(), &data.paymentid.clone()).await {
      Ok(c) => {
        approved = c.status.developer_approved;
        completed = c.status.developer_completed;
        cancelled = c.status.cancelled;
        usercancelled = c.status.user_cancelled;
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
      println!("PiPaymentFound, cancelled: {} - {} ", _pi_username.clone(), data.paymentid.clone());
      let err_type = format!(
        "Pi Server: payment cancelled: user {}, paymentid {}",
        &_pi_username, &data.paymentid
      );
      return Err(LemmyError::from_message(&err_type));
    }

    if !approved {
      println!("PiPaymentFound, do approve: {} - {} ", _pi_username.clone(), data.paymentid.clone());
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
        println!("PiPaymentFound, do complete: {} - {} ", _pi_username.clone(), data.paymentid.clone());
        let tx = dto_read.transaction.clone();
        match tx {
          Some(_tx) => {
            txid = _tx.txid;
            txlink = _tx._link;
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
      notes
    );


    if !exist {
      let mut payment_form = PiPaymentInsertForm::builder()
        //.domain(approve.domain.clone())
        .person_id( None)
        //.obj_cat(None)     //"".to_string(),
        //.obj_id( object_id.clone())
        //.notes( Some(_comment)) 
        .comment( Some(_comment))
        .ref_id( object_id.clone())
        .testnet( settings.pinetwork.pi_testnet)
        .finished( finished)
        .updated( updated)
        .pi_uid( _pi_uid)         //data.pi_uid
        .pi_username( _pi_username) //data.pi_username.clone(), Hide user name
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
      match _payment_dto.transaction {
        Some(tx) => {
          txlink = tx._link;          
          txid = tx.txid;
        }
        None => {}
      }
      let mut payment_form = PiPaymentUpdateForm::builder()
              .completed(completed)
              .approved(approved)
              .cancelled(cancelled)
              .user_cancelled(usercancelled)
              .tx_id(txid)
              .tx_link(txlink)
              .build();
      _payment = match PiPayment::update(context.pool(), payment_id, &payment_form).await
      {
        Ok(payment) => Some(payment),
        Err(_e) => {
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
