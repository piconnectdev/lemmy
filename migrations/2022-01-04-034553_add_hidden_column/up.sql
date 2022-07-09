alter table community add column hidden boolean default false;


create table mod_hide_community
(
    id uuid NOT NULL DEFAULT next_uuid() primary key,
    community_id uuid references community on update cascade on delete cascade not null,
    mod_person_id uuid references person on update cascade on delete cascade not null,
    when_ timestamp not null default now(),
    reason text,
    hidden boolean default false
);

