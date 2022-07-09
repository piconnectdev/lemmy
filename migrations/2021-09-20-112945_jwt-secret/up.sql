-- generate a jwt secret
create extension if not exists pgcrypto;

create table secret(
  id uuid NOT NULL DEFAULT next_uuid() primary key,
  jwt_secret varchar not null default gen_random_uuid()
);

insert into secret default values;
