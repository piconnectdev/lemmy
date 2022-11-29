-- Author signs the post, use web3 key
-- Server signs the post, use web3 key

alter table post add column private_id uuid NOT NULL DEFAULT next_uuid();
alter table post add column auth_sign text;
alter table post add column srv_sign text;
alter table post add column tx text;
alter table post add column extras jsonb;

alter table comment add column private_id uuid NOT NULL DEFAULT next_uuid();
alter table comment add column auth_sign text;
alter table comment add column srv_sign text;
alter table comment add column tx text;
alter table comment add column extras jsonb;

alter table private_message add column secured text;
alter table private_message add column auth_sign text;
alter table private_message add column srv_sign text;
alter table private_message add column tx text;

DROP VIEW person_alias_1;
DROP VIEW person_alias_2;

create view person_alias_1 as select * from person;
create view person_alias_2 as select * from person;

DROP VIEW comment_alias_1;
create view comment_alias_1 as select * from comment;


