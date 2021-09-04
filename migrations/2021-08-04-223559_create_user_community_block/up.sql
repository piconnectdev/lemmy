create table person_block (
  id uuid NOT NULL DEFAULT next_uuid() primary key,
  person_id uuid references person on update cascade on delete cascade not null,
  target_id uuid references person on update cascade on delete cascade not null,
  published timestamp not null default now(),
  unique(person_id, target_id)
);

create table community_block (
  id uuid NOT NULL DEFAULT next_uuid() primary key,
  person_id uuid references person on update cascade on delete cascade not null,
  community_id uuid references community on update cascade on delete cascade not null,
  published timestamp not null default now(),
  unique(person_id, community_id)
);
