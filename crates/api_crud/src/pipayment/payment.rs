use actix_web::web::Data;
use lemmy_api_common::pipayment::*;
use lemmy_db_schema::{
  newtypes::{CommentId, *},
  source::{comment::*, pipayment::*, post::*, person::*, person_balance::PersonBalance, community::Community, site::Site},
  traits::{Crud, Signable } ,
  utils::naive_now, 
};
use lemmy_utils::{error::LemmyError, settings::SETTINGS};
use lemmy_api_common::{context::LemmyContext, };

use uuid::Uuid;

use super::client::{pi_payment, pi_approve, pi_complete};

#[derive(Clone, Debug)]
pub struct PiPaymentInfo {
  pub domain: Option<String>,  
  pub pi_username: String,
  pub pi_uid: Option<PiUserId>,
  pub pi_token: Option<String>,
  pub obj_cat: Option<String>,
  pub obj_id: Option<Uuid>,
  pub ref_id: Option<Uuid>,
  pub paymentid: String,
  pub comment: Option<String>,
  pub auth: Option<String>,
}

pub async fn pi_payment_create(
  context: &Data<LemmyContext>,
  info: &PiPaymentInfo,
  pipayment: Option<PiPayment>,
  tx: Option<String>,
) -> Result<PiPayment, LemmyError> {

  let pi_username = info.pi_username.clone();
  let pi_uid = info.pi_uid.clone();
  let payment_id = info.paymentid.clone();  
  let mut person_id: Option<PersonId> = None;
  let mut verified = false;
  let person = match Person::find_by_extra_name(context.pool(), &pi_username.clone()).await
  {
    Ok(c) => {
      person_id = Some(c.id.clone());
      verified = c.verified;
      Some(c)
    },
    Err(_e) => None
  };

  let paytype ;
  if info.obj_cat.is_some()
  {
    paytype = info.obj_cat.clone().unwrap_or_default();
    if paytype == "reward"
    {
      match Person::find_by_name(context.pool(), &info.comment.clone().unwrap_or_default()).await
      {
        Ok(p) =>{
          if p.external_id.is_none() {
            return Err(LemmyError::from_message("Cannot approve reward the user")); 
          }
        },
        Err(_e) => {
          return Err(LemmyError::from_message("Cannot approve reward non-exist user")); 
        },
      };    
    }
  }

  let _pi_user_alias = pi_username;
  let mut _payment_id = format!("{}", payment_id);
  let _pi_uid = pi_uid;

  let mut exist = false;
  let mut fetch_pi_server = false;
  let mut approved = false;
  let mut completed = false;
  let mut finished = false;
  let mut cancelled = false;
  let mut usercancelled = false;
  let mut txid: String = "".to_string();
  let mut txlink: String = "".to_string();
  let mut dto: Option<PiPaymentDto> = None;

  let mut pid;
  let mut pmt;
  if pipayment.is_some() {
    return Err(LemmyError::from_message("Cannot approve"));    
  } else {
    fetch_pi_server = true;
  }
 
  dto = match pi_payment(context.client(), &_payment_id.clone()).await {
    Ok(c) => {
      approved = c.status.developer_approved;
      completed = c.status.developer_completed;
      cancelled = c.status.cancelled;
      usercancelled = c.status.user_cancelled;
      if c.transaction.is_some() {
        txid = c.transaction.clone().unwrap().txid;
      }
      //println!("pi_payment_create, fetch payment from server: {}", _payment_id.clone());
      Some(c)
    }
    Err(_e) => {
      // Pi Server error
      let err_str = format!(
        "Pi Server: error while check payment: user {}, paymentid {} error: {}",
        _pi_user_alias.clone(),
        _payment_id.clone(),
        _e.to_string()
      );
      return Err(LemmyError::from_message(&err_str));
    }
  };

  if !approved {
    if pipayment.is_some() {
      let err_str = format!(
        "The payment: user {}, paymentid {} was approved",
        _pi_user_alias.clone(),
        _payment_id.clone()
      );
      return Err(LemmyError::from_message(&err_str));
    } 
    dto = match pi_approve(context.client(), &payment_id).await {
      Ok(c) => { 
        println!("pi_payment_create, pi_approve: {}", _payment_id.clone());
        Some(c)
      },
      Err(_e) => {
        let err_str = format!(
          "Pi Server: approve paymentid {} error: {}",
          _payment_id.clone(),
          _e.to_string()
        );
        return Err(LemmyError::from_message(&err_str));
      },
    };
  } else if !completed {
    if tx.is_some() {
      txid = tx.unwrap();
    }
    dto = match pi_complete(context.client(), &payment_id, &txid).await {
      Ok(c) => {
        completed = true;
        //println!("pi_payment_create, {}", _payment_id.clone());
        Some(c)
      }
      Err(_e) => {
        let err_str = format!(
          "Pi Server: complete payment error {}, paymentid {} error: {}",
          _pi_user_alias.clone(),
          _payment_id.clone(),
          _e.to_string()
        );
        return Err(LemmyError::from_message(&err_str));
      },
    };
  }

  if !exist || !approved {
  } else {
  }

  let mut _payment_dto = PiPaymentDto {
    ..PiPaymentDto::default()
  };

  if dto.is_some() {
    // if completed && person.is_some() && !verified {
    //   match Person::update_kyced(context.pool(), person.unwrap().id).await {
    //     Ok(p) =>{
    //       println!("pi_payment_create, verify user {}", _pi_user_alias.clone());
    //     }
    //     Err(e) => {
    //       println!("pi_payment_create, verify user err {}", e.to_string());          
    //     }
    //   }
    // }
    _payment_dto = dto.unwrap();
  }

  let create_at = match chrono::NaiveDateTime::parse_from_str(&_payment_dto.created_at, "%Y-%m-%dT%H:%M:%S%.f%Z")
  {
      Ok(dt) => Some(dt),
      Err(_e) => {
        None
      }
  };

  completed = _payment_dto.status.developer_completed.clone();
  
  if !exist {
    println!("pi_payment_create, create local: {}", _payment_id.clone());
    let mut payment_form = PiPaymentInsertForm::builder()
      .domain(info.domain.clone())
      .instance_id(None)
      .person_id( person_id.clone())
      .obj_cat(info.obj_cat.clone())
      .obj_id(info.obj_id)
      .ref_id(info.ref_id)
      .a2u(0)
      .fee(0.0)
      .asset(Some("PI".to_string()))
      .comment(info.comment.clone())
      .testnet( context.settings().pinetwork.pi_testnet)
      
      .finished( false)
      .updated( None)
      .pi_uid( _pi_uid)
      .pi_username( _pi_user_alias.clone())      
      
      .identifier(Some(payment_id.clone()))
      .user_uid(Some(_payment_dto.user_uid))
      .amount( _payment_dto.amount)
      .memo(Some(_payment_dto.memo.clone()))
      .from_address(Some(_payment_dto.from_address))
      .to_address(Some(_payment_dto.to_address))
      .direction(Some(_payment_dto.direction))
      .network(Some(_payment_dto.network))
      .created_at(create_at)
      .approved(_payment_dto.status.developer_approved)
      .verified(_payment_dto.status.transaction_verified)
      .completed(_payment_dto.status.developer_completed)
      .cancelled(_payment_dto.status.cancelled)
      .user_cancelled(_payment_dto.status.user_cancelled)
      .tx_link(None)
      .tx_id(None)
      .tx_verified(false)
      .metadata(None) //_payment_dto.metadata,
      .extras(None)
      .build();

    match _payment_dto.transaction {
      Some(tx) => {
        payment_form.tx_link = Some(tx._link);
        payment_form.tx_verified = tx.verified;
        payment_form.tx_id = Some(tx.txid);
        payment_form.finished = true;
      }
      None => {}
    }
    let payment = match PiPayment::create(context.pool(), &payment_form).await
    {
      Ok(payment) => {
        pid = payment.id;
        println!("pi_payment_create create success: {} {}", _payment_id.clone(), pid.clone());
        Some(payment)
      }
      Err(_e) => {
        let err_str = _e.to_string();
        println!("pi_payment_create create error: {} {} ", _payment_id.clone(), err_str.clone());
        return Err(LemmyError::from_message(&err_str));
      }
    };
    pmt = payment.unwrap();
  } else {
    return Err(LemmyError::from_message("Approve exist payment"));
  }
  return Ok(pmt);
}

pub async fn pi_payment_update(
  context: &Data<LemmyContext>,
  info: &PiPaymentInfo,
  pipayment: Option<PiPayment>,
  tx: Option<String>,
) -> Result<PiPayment, LemmyError> {

  let pi_username = info.pi_username.clone();
  let pi_uid = info.pi_uid.clone();
  let payment_id = info.paymentid.clone();  
  let comment = info.comment.clone().unwrap_or("".to_string());
  let mut person_id: Option<PersonId> = None;
  let mut verified = false;

  let person = match Person::find_by_extra_name(context.pool(), &pi_username.clone()).await
  {
    Ok(c) => {
      person_id = Some(c.id.clone());
      verified = c.verified;
      Some(c)
    },
    Err(_e) => None
  };

  let _pi_user_alias = pi_username;

  let mut _payment_id = format!("{}", payment_id);
  let _pi_uid = pi_uid;

  let mut exist = false;
  let mut fetch_pi_server = false;

  let mut approved = false;
  let mut completed = false;
  let mut transaction_verified = false;
  let mut usercancelled = false;
  let mut cancelled = false;
  let mut txverified = false;
  let mut txid: Option<String> = None;
  let mut txlink: Option<String> = None;
  let mut from_address: Option<String> = None;
  let mut to_address: Option<String> = None;
  let mut finished = false;
  let mut step = 0;
  let mut dto: Option<PiPaymentDto> = None;

  let mut pid;
  let mut pmt;

  let mut amount: f64 = 0.0;
  let mut fee: f64 = 0.00;

  if pipayment.is_some() {
    let c = pipayment.clone().unwrap();
    exist = true;
    transaction_verified = c.tx_verified;
    approved = c.approved;
    completed = c.completed;
    cancelled = c.cancelled;
    txverified = c.tx_verified;
    txid = c.tx_id.clone();
    txlink = c.tx_link.clone();
    from_address = c.from_address.clone();
    to_address = c.to_address.clone();
    pid = c.id;
    amount = c.amount;
    step = c.step;
    if cancelled || completed {
      finished = true;
      fetch_pi_server = false;
    } else {
      fetch_pi_server = true;
    }
  } else {
    fetch_pi_server = true;
  }

  if fetch_pi_server && !finished {
    dto = match pi_payment(context.client(), &_payment_id.clone()).await {
      Ok(c) => {
        if c.transaction.is_some() {
          txid = Some(c.transaction.clone().unwrap().txid);
        }
        println!("pi_payment_update, fetch payment from server: {}", _payment_id.clone());
        if completed == true && c.status.developer_completed == true {
            return Err(LemmyError::from_message("Both side is completed, ignore"));
        }
        approved = c.status.developer_approved;
        completed = c.status.developer_completed;
        cancelled = c.status.cancelled;
        usercancelled = c.status.user_cancelled;
        transaction_verified = c.status.transaction_verified;
        from_address = Some(c.from_address.clone());
        to_address = Some(c.to_address.clone());
        amount = c.amount;

        if c.transaction.is_some() {
        let txdto = c.transaction.clone().unwrap_or_default();
          txverified = txdto.verified;
          txid = Some(txdto.txid);
          txlink = Some(txdto._link);        
        }
        Some(c)
      }
      Err(_e) => {
        // Pi Server error
        let err_str = format!(
          "Pi Server: error while check payment: user {}, paymentid {} error: {}",
          _pi_user_alias.clone(),
          _payment_id.clone(),
          _e.to_string()
        );
        return Err(LemmyError::from_message(&err_str));
      }
    };
  }

  if !approved {
    if pipayment.is_some() {
      let err_str = format!(
        "The payment: user {}, paymentid {} was approved",
        _pi_user_alias.clone(),
        _payment_id.clone()
      );
      return Err(LemmyError::from_message(&err_str));
    } 
    dto = match pi_approve(context.client(), &payment_id).await {
      Ok(c) => { 
        println!("pi_payment_update, pi_approve dto: {} {}", _payment_id.clone(), c.amount);
        approved = c.status.developer_approved;
        completed = c.status.developer_completed;
        cancelled = c.status.cancelled;
        usercancelled = c.status.user_cancelled;
        transaction_verified = c.status.transaction_verified;
        from_address = Some(c.from_address.clone());
        to_address = Some(c.to_address.clone());
        amount = c.amount;
        if c.transaction.is_some() {
        let txdto = c.transaction.clone().unwrap_or_default();
          txverified = txdto.verified;
          txid = Some(txdto.txid);
          txlink = Some(txdto._link);        
        }
        Some(c)
      },
      Err(_e) => {
        let err_str = format!(
          "Pi Server: approve payment error {}, paymentid {} error: {}",
          _pi_user_alias.clone(),
          _payment_id.clone(),
          _e.to_string()
        );
        return Err(LemmyError::from_message(&err_str));
      },
    };
  } else if !completed {
    if tx.is_some() {
      txid = tx.clone();
    }
    dto = match pi_complete(context.client(), &payment_id, &txid.clone().unwrap_or_default()).await {
      Ok(c) => {
        let dto_str = serde_json::to_string(&c.clone()).unwrap();
        println!("pi_payment_update, pi_complete dto: {} - {} {}", _pi_user_alias.clone(), _payment_id.clone(), &dto_str);
        approved = c.status.developer_approved;
        completed = c.status.developer_completed;
        cancelled = c.status.cancelled;
        usercancelled = c.status.user_cancelled;
        transaction_verified = c.status.transaction_verified;
        from_address = Some(c.from_address.clone());
        to_address = Some(c.to_address.clone());
        amount = c.amount;
        if c.transaction.is_some() {
        let txdto = c.transaction.clone().unwrap_or_default();
          txverified = txdto.verified;
          txid = Some(txdto.txid);
          txlink = Some(txdto._link);        
        }
        Some(c)
      }
      Err(_e) => {
        let err_str = format!(
          "Pi Server: complete payment error {}, paymentid {} error: {}",
          _pi_user_alias.clone(),
          _payment_id.clone(),
          _e.to_string()
        );
        return Err(LemmyError::from_message(&err_str));
      },
    };
  }

  if !exist || !approved {
  } else {
  }

  let mut _payment_dto = PiPaymentDto {
    ..PiPaymentDto::default()
  };

  if dto.is_some() {
    
    _payment_dto = dto.unwrap();
  }

  let create_at = match chrono::NaiveDateTime::parse_from_str(&_payment_dto.created_at, "%Y-%m-%dT%H:%M:%S%.f%Z")
  {
      Ok(dt) => Some(dt),
      Err(_e) => {
        None
      }
  };
  
  let object_id = info.obj_id.clone();
  if !exist {
    println!("pi_payment_update, create local {}", _payment_id.clone());
    let mut payment_form = PiPaymentInsertForm::builder()
      .domain(info.domain.clone())
      .instance_id(None)
      .person_id( person_id.clone())
      .obj_cat(info.obj_cat.clone())
      .obj_id(info.obj_id.clone())
      .a2u(0)
      .fee(0.0)
      .asset(Some("PI".to_string()))
      .ref_id(info.ref_id)
      .comment(info.comment.clone())
      .testnet(context.settings().pinetwork.pi_testnet)
      
      .finished(false)
      .updated(None)
      .pi_uid(_pi_uid)
      .pi_username( _pi_user_alias.clone())      
      
      .identifier(Some(payment_id.clone()))
      .user_uid(Some(_payment_dto.user_uid))
      .amount(_payment_dto.amount)
      .memo(Some(_payment_dto.memo.clone()))
      .from_address(Some(_payment_dto.from_address))
      .to_address(Some(_payment_dto.to_address))
      .direction(Some(_payment_dto.direction))
      .network(Some(_payment_dto.network))
      .created_at(create_at)
      .approved(_payment_dto.status.developer_approved)
      .verified(_payment_dto.status.transaction_verified)
      .completed(_payment_dto.status.developer_completed)
      .cancelled(_payment_dto.status.cancelled)
      .user_cancelled(_payment_dto.status.user_cancelled)
      .tx_link(None)
      .tx_id(None)
      .tx_verified(false)
      .metadata(None) //_payment_dto.metadata,
      .extras(None)
      .build();
    

    match _payment_dto.transaction {
      Some(tx) => {
        payment_form.tx_link = Some(tx._link);
        payment_form.tx_verified = tx.verified;
        payment_form.tx_id = Some(tx.txid);
        payment_form.finished = true;
      }
      None => {}
    }
    let payment = match PiPayment::create(context.pool(), &payment_form).await
    {
      Ok(payment) => {
        pid = payment.id;
        Some(payment)
      }
      Err(_e) => {
        let err_str = _e.to_string();
        return Err(LemmyError::from_message(&err_str));
      }
    };
    pmt = payment.unwrap();
  } else {
    println!("pi_payment_update, update local: {} - {} address:{}", _pi_user_alias.clone(), _payment_id.clone(), from_address.clone().unwrap_or_default()); 
    let mut payment_form = PiPaymentUpdateForm::builder()
        .step(step)
        .amount(_payment_dto.amount)
        .identifier(Some(payment_id.clone()))
        .user_uid(Some(_payment_dto.user_uid))
        .memo(Some(_payment_dto.memo.clone()))
        .from_address(from_address)
        .to_address(to_address)
        .direction(Some(_payment_dto.direction))
        .network(Some(_payment_dto.network))
        .approved(approved)
        .completed(completed)
        .cancelled(cancelled)
        .verified(transaction_verified)
        .user_cancelled(usercancelled)
        .tx_verified(txverified)
        .tx_link(txlink)
        .tx_id(txid)
        .updated(Some(naive_now()))
        .build();

    payment_form.finished = true;
    pid = pipayment.unwrap().id;
    let paytype = info.obj_cat.clone().unwrap_or_default();
    let mut ref_uuid;
    let payment = match PiPayment::update(context.pool(), pid, &payment_form).await
    {
      Ok(p) => {
        ref_uuid = p.ref_id.clone();
        Some(p)
      },
      Err(_e) => {
        let err_str = _e.to_string();
        println!("pi_payment_update, update error {}", &err_str.clone());
        return Err(LemmyError::from_message(&err_str));
      }
    };

    if completed && txverified {
      if person.is_some() && !verified {
        match Person::update_kyced(context.pool(), person_id.clone().unwrap()).await {
          Ok(p) =>{
          }
          Err(e) => {
          }
        }
      }
      if paytype == "deposit" {
        match PersonBalance::update_deposit(context.pool(), person_id.clone().unwrap(), amount).await
        {
          Ok(p) =>{},
          Err(_e) => {},
        };
      } else if paytype == "reward" {
        if person_id.is_some() {
          match PersonBalance::update_spent(context.pool(), person_id.clone().unwrap_or_default(), amount.clone(), fee).await
          {
            Ok(p) =>{},
            Err(_e) => {},
          };
        }
        if ref_uuid.is_some() {
          let uuid = ref_uuid.clone().unwrap();
          let person_tipped_id = PersonId(uuid);
          match PersonBalance::update_received(context.pool(), person_tipped_id, amount).await
          {
            Ok(p) =>{},
            Err(_e) => {},
          };
        }
      } else if paytype == "tip:page" {
        if person_id.is_some() {
          match PersonBalance::update_spent(context.pool(), person_id.clone().unwrap_or_default(), amount.clone(), fee).await
          {
            Ok(p) =>{},
            Err(_e) => {},
          };
        }
        if ref_uuid.is_some() {
          let uuid = ref_uuid.clone().unwrap();
          let person_tipped_id = PersonId(uuid);
          match PersonBalance::update_received(context.pool(), person_tipped_id, amount).await
          {
            Ok(p) =>{},
            Err(_e) => {},
          };
        }
      } else if paytype == "tip:note" {
        if person_id.is_some() {
          match PersonBalance::update_spent(context.pool(), person_id.clone().unwrap_or_default(), amount.clone(), fee).await
          {
            Ok(p) =>{},
            Err(_e) => {},
          };
        }
        if ref_uuid.is_some() {
          let uuid = ref_uuid.clone().unwrap();
          let person_tipped_id = PersonId(uuid);
          match PersonBalance::update_received(context.pool(), person_tipped_id, amount).await
          {
            Ok(p) =>{},
            Err(_e) => {},
          };
        }
      } else  {
        if person_id.is_some() {
          match PersonBalance::update_spent(context.pool(), person_id.clone().unwrap_or_default(), amount, fee).await
          {
            Ok(p) =>{},
            Err(_e) => {},
          };
        }
        if paytype == "page" {
        let link = payment_form.tx_link.clone();
        let uuid = object_id.clone();
        match uuid {
          Some(u) => {
            let post_id = PostId(u);
            let updated_post = match Post::update_tx(context.pool(), post_id, &_payment_id.clone(), &link.unwrap_or("".to_string())) .await
            {
              Ok(p) => {
                Some(p)
              }
              Err(_e) => None,
            };
          },
          None => {
            //None
          }
        };
        } else if paytype == "note" {
          let link = payment_form.tx_link.clone();
          let uuid = object_id.clone();
          match uuid {
            Some(u) => {
              let comment_id = CommentId(u);
              let updated_comment = match Comment::update_tx(context.pool(), comment_id, &_payment_id.clone(), &link.unwrap_or("".to_string())).await
              {
                Ok(c) => {
                  Some(c)
                }
                Err(_e) => None,
              };
            },
            None => {
            }
          };
        } else if paytype == "group" {
          let link = payment_form.tx_link.clone();
          let uuid = object_id.clone();
          match uuid {
            Some(u) => {
              let community_id = CommunityId(u);
              let updated_comment = match Community::update_tx(context.pool(), community_id, &_payment_id.clone(), &link.unwrap_or("".to_string())).await
              {
                Ok(c) => {
                  Some(c)
                }
                Err(_e) => None,
              };
            },
            None => {
            }
          };
        } else if paytype == "person" {
          let link = payment_form.tx_link.clone();
          let link2 = payment_form.tx_link.clone();
          let uuid = object_id.clone();
          match uuid {
            Some(u) => {
              let person_id = PersonId(u);
              let updated_comment = match Person::update_tx(context.pool(), person_id, &_payment_id.clone(), &link.unwrap_or("".to_string())).await
              {
                Ok(c) => {
                  Some(c)
                }
                Err(_e) => None,
              };
            },
            None => {
            }
          };
        } else if paytype == "site" {
          let link = payment_form.tx_link.clone();
          let link2 = payment_form.tx_link.clone();
          let uuid = object_id.clone();
          match uuid {
            Some(u) => {
              let site_id = SiteId(u);
              let updated_comment = match Site::update_tx(context.pool(), site_id, &_payment_id.clone(), &link.unwrap_or("".to_string())).await
              {
                Ok(c) => {
                  Some(c)
                }
                Err(_e) => None,
              };
            },
            None => {
            }
          };
        }
      }      
    }
    pmt = payment.unwrap();
  }
  return Ok(pmt);
}
