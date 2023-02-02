create table pipayment (
  id uuid NOT NULL DEFAULT next_uuid(),
  domain text,
  instance_id uuid,
  person_id uuid,
  obj_cat uuid,
  obj_id uuid,    
  ref_id uuid,  
  testnet bool,
  finished bool default false,
  published timestamp not null default now(),
  updated timestamp,
  comment text,  
  pi_uid uuid,  
  pi_username text,
  identifier text,
  user_uid text,
  amount double precision,
  memo text,
  from_address text,
  to_address text,
  direction text,
  created_at timestamp,
  approved bool,
  verified bool,
  completed bool,
  cancelled bool,
  user_cancelled bool,
  tx_verified bool,
  tx_id text,  
  tx_link text,
  network text,
  metadata jsonb,
  extras jsonb,
  primary key(id, published)
  -- CONSTRAINT pipayment_identifier_unique UNIQUE (identifier)
);

create index idx_pipayment_domain on pipayment (domain);
create index idx_pipayment_instance_id on pipayment (instance_id);
create index idx_pipayment_obj_cat on pipayment (obj_cat);
create index idx_pipayment_obj_id on pipayment (obj_id);
create index idx_pipayment_creator on pipayment (person_id);
create index idx_pipayment_pi_username on pipayment (pi_username);
create index idx_pipayment_pi_uid on pipayment (pi_uid);
create index idx_pipayment_user_uid on pipayment (user_uid);
create index idx_pipayment_identifier on pipayment (identifier);
create index idx_pipayment_memo on pipayment (memo);

create table person_balances {
  id uuid NOT NULL DEFAULT next_uuid() primary key,
  person_id uuid,
  published timestamp not null default now(),
  asset_name text,
  total_deposit double precision,
  total_withdraw double precision,
  amount double precision,
  pending double precision,
  extras jsonb,
};

create index idx_person_balances_person_id on person_balances (person_id);

create table person_withdraw {
  id uuid NOT NULL DEFAULT next_uuid() primary key,
  person_id uuid,
  published timestamp not null default now(),
  asset_name text,
  finished bool,
  current_amount double precision,
  amount double precision,
  remain double precision,
  stat text,
  txid text;
  link text;
  updated timestamp,
  extras jsonb,
};

create index idx_person_withdraw_person_id on person_balances (person_id);