create table pipayment (
  id uuid NOT NULL DEFAULT next_uuid() primary key,
  person_id uuid,  
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
  extras jsonb
);

alter table person add column extra_user_id text;

