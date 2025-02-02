-- create user follower table with two references to persons
create table person_follower (
    id uuid NOT NULL DEFAULT next_uuid() primary key,
    person_id uuid references person on update cascade on delete cascade not null,
    follower_id uuid references person on update cascade on delete cascade not null,
    published timestamp not null default now(),
    pending boolean not null,
    unique (follower_id, person_id)
);

update community_follower set pending = false where pending is null;
alter table community_follower alter column pending set not null;
