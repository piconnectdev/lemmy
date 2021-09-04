-- Add the mod_transfer_community log table
create table mod_transfer_community (
  id uuid NOT NULL DEFAULT next_uuid() primary key,
  mod_person_id uuid references person on update cascade on delete cascade not null,
  other_person_id uuid references person on update cascade on delete cascade not null,
  community_id uuid references community on update cascade on delete cascade not null,
  removed boolean default false,
  when_ timestamp not null default now()
);
