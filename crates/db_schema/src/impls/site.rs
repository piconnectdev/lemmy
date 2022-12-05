use crate::{
  newtypes::{DbUrl, SiteId},
  schema::site::dsl::{actor_id, id, site},
  source::{
    actor_language::SiteLanguage,
    site::{Site, SiteInsertForm, SiteUpdateForm},
  },
  traits::{Crud, Signable},
  utils::{get_conn, DbPool},
};
use diesel::{dsl::insert_into, result::Error, ExpressionMethods, QueryDsl};
use diesel_async::RunQueryDsl;
use url::Url;
use sha2::{Digest, Sha256};

#[async_trait]
impl Crud for Site {
  type InsertForm = SiteInsertForm;
  type UpdateForm = SiteUpdateForm;
  type IdType = SiteId;

  async fn read(pool: &DbPool, _site_id: SiteId) -> Result<Self, Error> {
    let conn = &mut get_conn(pool).await?;
    site.first::<Self>(conn).await
  }

  async fn create(pool: &DbPool, form: &Self::InsertForm) -> Result<Self, Error> {
    let conn = &mut get_conn(pool).await?;
    let site_ = insert_into(site)
      .values(form)
      .on_conflict(actor_id)
      .do_update()
      .set(form)
      .get_result::<Self>(conn)
      .await?;

    // initialize with all languages
    SiteLanguage::update(pool, vec![], &site_).await?;
    Ok(site_)
  }

  async fn update(
    pool: &DbPool,
    site_id: SiteId,
    new_site: &Self::UpdateForm,
  ) -> Result<Self, Error> {
    let conn = &mut get_conn(pool).await?;
    diesel::update(site.find(site_id))
      .set(new_site)
      .get_result::<Self>(conn)
      .await
  }

  async fn delete(pool: &DbPool, site_id: SiteId) -> Result<usize, Error> {
    let conn = &mut get_conn(pool).await?;
    diesel::delete(site.find(site_id)).execute(conn).await
  }
}

impl Site {
  pub async fn read_from_apub_id(pool: &DbPool, object_id: Url) -> Result<Option<Self>, Error> {
    let conn = &mut get_conn(pool).await?;
    let object_id: DbUrl = object_id.into();
    Ok(
      site
        .filter(actor_id.eq(object_id))
        .first::<Site>(conn)
        .await
        .ok()
        .map(Into::into),
    )
  }

  // TODO this needs fixed
  pub async fn read_remote_sites(pool: &DbPool) -> Result<Vec<Self>, Error> {
    let conn = &mut get_conn(pool).await?;
    site.order_by(id).offset(1).get_results::<Self>(conn).await
  }

/// Instance actor is at the root path, so we simply need to clear the path and other unnecessary
/// parts of the url.
  pub fn instance_actor_id_from_url(mut url: Url) -> Url {
    url.set_fragment(None);
    url.set_path("");
    url.set_query(None);
    url
  }
}

#[async_trait]
impl Signable for Site {
  type Form = Site;
  type IdType = SiteId;

  async fn update_srv_sign(
    pool: &DbPool,
    site_id: SiteId,
    sig: &str,
  ) -> Result<Self, Error> {
    use crate::schema::site::dsl::*;
    let conn = &mut get_conn(pool).await?;
    diesel::update(site.find(site_id))
      .set(srv_sign.eq(sig))
      .get_result::<Self>(conn)
      .await
  }

  async fn update_tx(
    pool: &DbPool,
    site_id: SiteId,
    txlink: &str,
  ) -> Result<Self, Error> {
    use crate::schema::site::dsl::*;
    let conn = &mut get_conn(pool).await?;
    diesel::update(site.find(site_id))
      .set(tx.eq(txlink))
      .get_result::<Self>(conn)
      .await
  }

  async fn sign_data(data: &Site) -> (Option<String>, Option<String>, Option<String>) {    
    let mut sha_meta = Sha256::new();
    let mut sha_content = Sha256::new();
    let mut sha256 = Sha256::new();

    let meta_data = format!("{};{};{};", data.id.clone().0.simple(), data.actor_id.clone().to_string(), data.published.clone().to_string());

    sha_meta.update(format!("{}",meta_data));
    let meta:  String = format!("{:x}", sha_meta.finalize());

    sha_content.update(data.name.clone().clone());
    let content:  String = format!("{:x}", sha_content.finalize());

    sha256.update(meta.clone());
    sha256.update(content.clone());
    let message: String = format!("{:x}", sha256.finalize());

    //let meta = lemmy_utils::utils::eth_sign_message(meta);
    //let content = lemmy_utils::utils::eth_sign_message(content);
    let signature = lemmy_utils::utils::eth_sign_message(message);
    return (signature, Some(meta_data), Some(content));
  
  }

}