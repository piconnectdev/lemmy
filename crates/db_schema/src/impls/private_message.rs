use crate::{
  newtypes::{DbUrl, PersonId, PrivateMessageId},
  schema::private_message::dsl::{ap_id, private_message, read, recipient_id},
  source::private_message::{PrivateMessage, PrivateMessageInsertForm, PrivateMessageUpdateForm},
  traits::{Crud, DeleteableOrRemoveable, Signable},
  utils::{get_conn, DbPool},
};
use diesel::{dsl::insert_into, result::Error, ExpressionMethods, QueryDsl};
use diesel_async::RunQueryDsl;
use lemmy_utils::error::LemmyError;
use url::Url;
use sha2::{Digest, Sha256};

#[async_trait]
impl Crud for PrivateMessage {
  type InsertForm = PrivateMessageInsertForm;
  type UpdateForm = PrivateMessageUpdateForm;
  type IdType = PrivateMessageId;
  async fn read(pool: &DbPool, private_message_id: PrivateMessageId) -> Result<Self, Error> {
    let conn = &mut get_conn(pool).await?;
    private_message
      .find(private_message_id)
      .first::<Self>(conn)
      .await
  }

  async fn create(pool: &DbPool, form: &Self::InsertForm) -> Result<Self, Error> {
    let conn = &mut get_conn(pool).await?;
    insert_into(private_message)
      .values(form)
      .on_conflict(ap_id)
      .do_update()
      .set(form)
      .get_result::<Self>(conn)
      .await
  }

  async fn update(
    pool: &DbPool,
    private_message_id: PrivateMessageId,
    form: &Self::UpdateForm,
  ) -> Result<Self, Error> {
    let conn = &mut get_conn(pool).await?;
    diesel::update(private_message.find(private_message_id))
      .set(form)
      .get_result::<Self>(conn)
      .await
  }
  async fn delete(pool: &DbPool, pm_id: Self::IdType) -> Result<usize, Error> {
    let conn = &mut get_conn(pool).await?;
    diesel::delete(private_message.find(pm_id))
      .execute(conn)
      .await
  }
}

impl PrivateMessage {
  pub async fn mark_all_as_read(
    pool: &DbPool,
    for_recipient_id: PersonId,
  ) -> Result<Vec<PrivateMessage>, Error> {
    let conn = &mut get_conn(pool).await?;
    diesel::update(
      private_message
        .filter(recipient_id.eq(for_recipient_id))
        .filter(read.eq(false)),
    )
    .set(read.eq(true))
    .get_results::<Self>(conn)
    .await
  }

  pub async fn read_from_apub_id(
    pool: &DbPool,
    object_id: Url,
  ) -> Result<Option<Self>, LemmyError> {
    let conn = &mut get_conn(pool).await?;
    let object_id: DbUrl = object_id.into();
    Ok(
      private_message
        .filter(ap_id.eq(object_id))
        .first::<PrivateMessage>(conn)
        .await
        .ok()
        .map(Into::into),
    )
  }
}

#[async_trait]
impl Signable for PrivateMessage { 
  type Form = PrivateMessage;
  type IdType = PrivateMessageId;
  async fn update_srv_sign(
    pool: &DbPool,
    private_message_id: PrivateMessageId,
    sig: &str,
  ) -> Result<Self, Error> {
    use crate::schema::private_message::dsl::*;
    let conn = &mut get_conn(pool).await?;
    diesel::update(private_message.find(private_message_id))
      .set(srv_sign.eq(sig))
      .get_result::<Self>(conn)
      .await
  }

  async fn update_tx(
    pool: &DbPool,
    private_message_id: PrivateMessageId,
    txlink: &str,
  ) -> Result<Self, Error> {
    use crate::schema::private_message::dsl::*;
    let conn = &mut get_conn(pool).await?;
    diesel::update(private_message.find(private_message_id))
      .set(tx.eq(txlink))
      .get_result::<Self>(conn)
      .await
  }

  async fn sign_data(updated_message: &PrivateMessage) -> (Option<String>, Option<String>, Option<String>) {    
    let mut sha_meta = Sha256::new();
    let mut sha_content = Sha256::new();
    let mut sha256 = Sha256::new();

    sha_meta.update(format!("{}",updated_message.id.clone().0.simple()));
    sha_meta.update(format!("{}",updated_message.creator_id.clone().0.simple()));
    sha_meta.update(format!("{}",updated_message.recipient_id.clone().0.simple()));
    let meta:  String = format!("{:x}", sha_meta.finalize());

    sha_content.update(updated_message.secured.clone().unwrap_or_default().clone());
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

impl DeleteableOrRemoveable for PrivateMessage {
  fn blank_out_deleted_or_removed_info(mut self) -> Self {
    self.content = String::new();
    self
  }
}

#[cfg(test)]
mod tests {
  use crate::{
    source::{
      instance::Instance,
      person::{Person, PersonInsertForm},
      private_message::{PrivateMessage, PrivateMessageInsertForm, PrivateMessageUpdateForm},
    },
    traits::Crud,
    utils::build_db_pool_for_tests,
  };
  use serial_test::serial;

  #[tokio::test]
  #[serial]
  async fn test_crud() {
    let pool = &build_db_pool_for_tests().await;

    let inserted_instance = Instance::create(pool, "my_domain.tld").await.unwrap();

    let creator_form = PersonInsertForm::builder()
      .name("creator_pm".into())
      .public_key("pubkey".to_string())
      .instance_id(inserted_instance.id)
      .build();

    let inserted_creator = Person::create(pool, &creator_form).await.unwrap();

    let recipient_form = PersonInsertForm::builder()
      .name("recipient_pm".into())
      .public_key("pubkey".to_string())
      .instance_id(inserted_instance.id)
      .build();

    let inserted_recipient = Person::create(pool, &recipient_form).await.unwrap();

    let private_message_form = PrivateMessageInsertForm::builder()
      .content("A test private message".into())
      .creator_id(inserted_creator.id)
      .recipient_id(inserted_recipient.id)
      .build();

    let inserted_private_message = PrivateMessage::create(pool, &private_message_form)
      .await
      .unwrap();

    let expected_private_message = PrivateMessage {
      id: inserted_private_message.id,
      content: "A test private message".into(),
      creator_id: inserted_creator.id,
      recipient_id: inserted_recipient.id,
      deleted: false,
      read: false,
      updated: None,
      published: inserted_private_message.published,
      ap_id: inserted_private_message.ap_id.clone(),
      local: true,
      secured: None,
      auth_sign: None, 
      srv_sign: None,
      tx: None,
    };

    let read_private_message = PrivateMessage::read(pool, inserted_private_message.id)
      .await
      .unwrap();

    let private_message_update_form = PrivateMessageUpdateForm::builder()
      .content(Some("A test private message".into()))
      .build();
    let updated_private_message = PrivateMessage::update(
      pool,
      inserted_private_message.id,
      &private_message_update_form,
    )
    .await
    .unwrap();

    let deleted_private_message = PrivateMessage::update(
      pool,
      inserted_private_message.id,
      &PrivateMessageUpdateForm::builder()
        .deleted(Some(true))
        .build(),
    )
    .await
    .unwrap();
    let marked_read_private_message = PrivateMessage::update(
      pool,
      inserted_private_message.id,
      &PrivateMessageUpdateForm::builder().read(Some(true)).build(),
    )
    .await
    .unwrap();
    Person::delete(pool, inserted_creator.id).await.unwrap();
    Person::delete(pool, inserted_recipient.id).await.unwrap();
    Instance::delete(pool, inserted_instance.id).await.unwrap();

    assert_eq!(expected_private_message, read_private_message);
    assert_eq!(expected_private_message, updated_private_message);
    assert_eq!(expected_private_message, inserted_private_message);
    assert!(deleted_private_message.deleted);
    assert!(marked_read_private_message.read);
  }
}
