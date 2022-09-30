use crate::{newtypes::{DbUrl, SiteId}, source::site::*, traits::Crud};
use diesel::{dsl::*, result::Error, *};
use url::Url;
use sha2::{Digest, Sha256};

impl Crud for Site {
  type Form = SiteForm;
  type IdType = SiteId;
  fn read(conn: &mut PgConnection, _site_id: SiteId) -> Result<Self, Error> {
    use crate::schema::site::dsl::*;
    site.first::<Self>(conn)
  }

  fn create(conn: &mut PgConnection, new_site: &SiteForm) -> Result<Self, Error> {
    use crate::schema::site::dsl::*;
    insert_into(site).values(new_site).get_result::<Self>(conn)
  }

  fn update(conn: &mut PgConnection, site_id: SiteId, new_site: &SiteForm) -> Result<Self, Error> {
    use crate::schema::site::dsl::*;
    diesel::update(site.find(site_id))
      .set(new_site)
      .get_result::<Self>(conn)
  }
  fn delete(conn: &mut PgConnection, site_id: SiteId) -> Result<usize, Error> {
    use crate::schema::site::dsl::*;
    diesel::delete(site.find(site_id)).execute(conn)
  }
}

impl Site {
  pub fn read_local_site(conn: &mut PgConnection) -> Result<Self, Error> {
    use crate::schema::site::dsl::*;
    site.order_by(id).first::<Self>(conn)
  }

  pub fn upsert(conn: &mut PgConnection, site_form: &SiteForm) -> Result<Site, Error> {
    use crate::schema::site::dsl::*;
    insert_into(site)
      .values(site_form)
      .on_conflict(actor_id)
      .do_update()
      .set(site_form)
      .get_result::<Self>(conn)
  }

  pub fn read_from_apub_id(conn: &mut PgConnection, object_id: Url) -> Result<Option<Self>, Error> {
    use crate::schema::site::dsl::*;
    let object_id: DbUrl = object_id.into();
    Ok(
      site
        .filter(actor_id.eq(object_id))
        .first::<Site>(conn)
        .ok()
        .map(Into::into),
    )
  }

  pub fn read_remote_sites(conn: &mut PgConnection) -> Result<Vec<Self>, Error> {
    use crate::schema::site::dsl::*;
    site.order_by(id).offset(1).get_results::<Self>(conn)
  }

  pub fn update_srv_sign(
    conn: &mut PgConnection,
    site_id: SiteId,
    sig: &str,
  ) -> Result<Self, Error> {
    use crate::schema::site::dsl::*;
    diesel::update(site.find(site_id))
      .set(srv_sign.eq(sig))
      .get_result::<Self>(conn)
  }

  pub fn sign_data(data: &Site) -> (Option<String>, Option<String>, Option<String>) {    
    let mut sha_meta = Sha256::new();
    let mut sha_content = Sha256::new();
    let mut sha256 = Sha256::new();

    sha_meta.update(format!("{}",data.id.clone().0.simple()));
    sha_meta.update(format!("{}",data.actor_id.clone().to_string()));
    //sha_meta.update(format!("{}",data.published.clone().to_string()));
    let meta:  String = format!("{:x}", sha_meta.finalize());

    sha_content.update(data.name.clone().clone());
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
