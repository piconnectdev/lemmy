-- alter table site add column cert text;
-- alter table site add column tx text;

DROP VIEW person_alias_1;
DROP VIEW person_alias_2;

alter table person add column web3_address text;
alter table person add column sol_address text;
alter table person add column dap_address text;
alter table person add column cert text;
alter table person add column tx text;

create view person_alias_1 as select * from person;
create view person_alias_2 as select * from person;

-- alter table person add column extras jsonb;

alter table community add column cert text;
alter table community add column tx text;

alter table post add column cert text;

DROP VIEW comment_alias_1;
alter table comment add column cert text;
create view comment_alias_1 as select * from comment;

alter table private_message add column secured text;
alter table private_message add column cert text;
alter table private_message add column tx text;

