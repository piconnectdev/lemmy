create table tagline (
  id uuid NOT NULL DEFAULT next_uuid() primary key,
  local_site_id uuid references local_site on update cascade on delete cascade not null,
  content text not null,
  published timestamp without time zone default now() not null,
  updated timestamp without time zone
);