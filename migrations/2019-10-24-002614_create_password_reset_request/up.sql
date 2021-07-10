create table password_reset_request (
  id bigserial primary key,
  user_id bigint references user_ on update cascade on delete cascade not null,
  token_encrypted text not null,
  published timestamp not null default now()
);
