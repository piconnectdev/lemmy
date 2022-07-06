alter table person add column verified bool DEFAULT false;
alter table person add column private_seeds text;
alter table person add column pi_address text;

alter table post add column private_id uuid NOT NULL DEFAULT next_uuid();
alter table post add column tx text;
alter table post add column pi_tx text;
alter table post add column extras jsonb;

alter table comment add column private_id uuid NOT NULL DEFAULT next_uuid();
alter table comment add column tx text;
alter table comment add column pi_tx text;
alter table comment add column extras jsonb;