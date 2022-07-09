-- Add the admin_purge tables

create table admin_purge_person (
  id uuid NOT NULL DEFAULT next_uuid() primary key,
  admin_person_id uuid references person on update cascade on delete cascade not null,
  reason text,
  when_ timestamp not null default now()
);

create table admin_purge_community (
  id uuid NOT NULL DEFAULT next_uuid() primary key,
  admin_person_id uuid references person on update cascade on delete cascade not null,
  reason text,
  when_ timestamp not null default now()
);

create table admin_purge_post (
  id uuid NOT NULL DEFAULT next_uuid() primary key,
  admin_person_id uuid references person on update cascade on delete cascade not null,
  community_id uuid references community on update cascade on delete cascade not null,
  reason text,
  when_ timestamp not null default now()
);

create table admin_purge_comment (
  id uuid NOT NULL DEFAULT next_uuid() primary key,
  admin_person_id uuid references person on update cascade on delete cascade not null,
  post_id uuid references post on update cascade on delete cascade not null,
  reason text,
  when_ timestamp not null default now()
);
