create table pipayment (
  id uuid NOT NULL DEFAULT next_uuid() primary key,
  domain text
  instance_id uuid,
  person_id uuid,  
  testnet bool,
  published timestamp not null default now(),
  object_cat text,  
  object_id uuid,

  pi_username text,
  pi_uid uuid,  
  finished bool default false,
  updated timestamp,
  other_id uuid,

  identifier text,
  user_uid text,
  amount double precision,
  memo text,
  to_address text,
  created_at timestamp,
  approved bool,
  verified bool,
  completed bool,
  cancelled bool,
  user_cancelled bool,
  tx_verified bool,
  tx_id text,  
  tx_link text,
  metadata jsonb,
  extras jsonb,
  notes text,

  CONSTRAINT pipayment_identifier_unique UNIQUE (identifier)
);

-- create index idx_pipayment_domain on pipayment (domain);
-- create index idx_pipayment_instance_id on pipayment (instance_id);
-- create index idx_pipayment_creator_id on pipayment (person_id);
-- create index idx_pipayment_object_cat on pipayment (object_cat);
-- create index idx_pipayment_object_id on pipayment (object_id);

-- create index idx_pipayment_pi_username on pipayment (pi_username);
-- create index idx_pipayment_pi_uid on pipayment (pi_uid);

-- create index idx_pipayment_user_uid on pipayment (user_uid);
-- create index idx_pipayment_memo on pipayment (memo);
-- create index idx_pipayment_identifier on pipayment (identifier);
-- create index idx_pipayment_to_address on pipayment (to_address);