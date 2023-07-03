use crate::structs::{AdminPurgeCommentView, ModlogListParams};
use diesel::{
  result::Error,
  BoolExpressionMethods,
  ExpressionMethods,
  IntoSql,
  JoinOnDsl,
  NullableExpressionMethods,
  QueryDsl,
};
use diesel_async::RunQueryDsl;
use lemmy_db_schema::{
  newtypes::PersonId,
  schema::{admin_purge_comment, person, post},
  source::{moderator::AdminPurgeComment, person::Person, post::Post},
  traits::JoinView,
  utils::{get_conn, limit_and_offset, DbPool},
};

type AdminPurgeCommentViewTuple = (AdminPurgeComment, Option<Person>, Post);

impl AdminPurgeCommentView {
  pub async fn list(pool: &DbPool, params: ModlogListParams) -> Result<Vec<Self>, Error> {
    let conn = &mut get_conn(pool).await?;
    let uuid = uuid::Uuid::parse_str("00000000-0000-0000-0000-000000000000").unwrap();
    let admin_person_id_join = params.mod_person_id.unwrap_or(PersonId(uuid.clone()));
    let show_mod_names = !params.hide_modlog_names;
    let show_mod_names_expr = show_mod_names.as_sql::<diesel::sql_types::Bool>();

    let admin_names_join = admin_purge_comment::admin_person_id
      .eq(person::id)
      .and(show_mod_names_expr.or(person::id.eq(admin_person_id_join)));

    let mut query = admin_purge_comment::table
      .left_join(person::table.on(admin_names_join))
      .inner_join(post::table)
      .select((
        admin_purge_comment::all_columns,
        person::all_columns.nullable(),
        post::all_columns,
      ))
      .into_boxed();

    if let Some(admin_person_id) = params.mod_person_id {
      query = query.filter(admin_purge_comment::admin_person_id.eq(admin_person_id));
    };

    let (limit, offset) = limit_and_offset(params.page, params.limit)?;

    let res = query
      .limit(limit)
      .offset(offset)
      .order_by(admin_purge_comment::when_.desc())
      .load::<AdminPurgeCommentViewTuple>(conn)
      .await?;

    let results = res.into_iter().map(Self::from_tuple).collect();
    Ok(results)
  }
}

impl JoinView for AdminPurgeCommentView {
  type JoinTuple = AdminPurgeCommentViewTuple;
  fn from_tuple(a: Self::JoinTuple) -> Self {
    Self {
      admin_purge_comment: a.0,
      admin: a.1,
      post: a.2,
    }
  }
}
