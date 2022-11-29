-- Actors, see https://join-lemmy.org/docs/en/federation/lemmy_protocol.html#actors
alter table site add column admin_address text;
alter table site add column signer_address text;
alter table site add column auth_sign text;
alter table site add column srv_sign text;
alter table site add column tx text;

alter table community add column auth_sign text;
alter table community add column srv_sign text;
alter table community add column tx text;

alter table person add column external_id text DEFAULT NULL, ADD CONSTRAINT person_external_id_key UNIQUE (external_id);
alter table person add column private_seeds text;
alter table person add column verified bool DEFAULT false;
alter table person add column pi_address text;
alter table person add column web3_address text;
alter table person add column dap_address text;
alter table person add column cosmos_address text;
alter table person add column sui_address text;
alter table person add column ton_address text;
alter table person add column pol_address text;
alter table person add column extras jsonb;

alter table person add column auth_sign text;
alter table person add column srv_sign text;
alter table person add column tx text;
