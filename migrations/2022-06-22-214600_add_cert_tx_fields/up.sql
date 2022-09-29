-- alter table site add column auth_sign text;
-- alter table site add column admin_address text;
-- alter table site add column signer_address text;
-- alter table site add column tx text;
alter table site add column srv_sign text;

alter table person add column cert text;
alter table person add column auth_sign text;
alter table person add column srv_sign text;

alter table community add column cert text;
alter table community add column auth_sign text;
alter table community add column srv_sign text;
alter table community add column tx text;

-- Author signs the post, use web3 key
-- Server signs the post, use web3 key
alter table post add column cert text;
alter table post add column auth_sign text;
alter table post add column srv_sign text;
alter table comment add column cert text;
alter table comment add column auth_sign text;
alter table comment add column srv_sign text;

alter table private_message add column secured text;
alter table private_message add column cert text;
alter table private_message add column auth_sign text;
alter table private_message add column srv_sign text;
alter table private_message add column tx text;

DROP VIEW person_alias_1;
DROP VIEW person_alias_2;

create view person_alias_1 as select * from person;
create view person_alias_2 as select * from person;

DROP VIEW comment_alias_1;
create view comment_alias_1 as select * from comment;


