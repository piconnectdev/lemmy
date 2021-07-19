create table mod_remove_post (
  id uuid NOT NULL DEFAULT next_uuid() primary key,
  mod_user_id uuid references user_ on update cascade on delete cascade not null,
  post_id uuid references post on update cascade on delete cascade not null,
  reason text,
  removed boolean default true,
  when_ timestamp not null default now()
);

create table mod_lock_post (
  id uuid NOT NULL DEFAULT next_uuid() primary key,
  mod_user_id uuid references user_ on update cascade on delete cascade not null,
  post_id uuid references post on update cascade on delete cascade not null,
  locked boolean default true,
  when_ timestamp not null default now()
);

create table mod_remove_comment (
  id uuid NOT NULL DEFAULT next_uuid() primary key,
  mod_user_id uuid references user_ on update cascade on delete cascade not null,
  comment_id uuid references comment on update cascade on delete cascade not null,
  reason text,
  removed boolean default true,
  when_ timestamp not null default now()
);

create table mod_remove_community (
  id uuid NOT NULL DEFAULT next_uuid() primary key,
  mod_user_id uuid references user_ on update cascade on delete cascade not null,
  community_id uuid references community on update cascade on delete cascade not null,
  reason text,
  removed boolean default true,
  expires timestamp,
  when_ timestamp not null default now()
);

-- TODO make sure you can't ban other mods
create table mod_ban_from_community (
  id uuid NOT NULL DEFAULT next_uuid() primary key,
  mod_user_id uuid references user_ on update cascade on delete cascade not null,
  other_user_id uuid references user_ on update cascade on delete cascade not null,
  community_id uuid references community on update cascade on delete cascade not null,
  reason text,
  banned boolean default true,
  expires timestamp,
  when_ timestamp not null default now()
);

create table mod_ban (
  id uuid NOT NULL DEFAULT next_uuid() primary key,
  mod_user_id uuid references user_ on update cascade on delete cascade not null,
  other_user_id uuid references user_ on update cascade on delete cascade not null,
  reason text,
  banned boolean default true,
  expires timestamp,
  when_ timestamp not null default now()
);

create table mod_add_community (
  id uuid NOT NULL DEFAULT next_uuid() primary key,
  mod_user_id uuid references user_ on update cascade on delete cascade not null,
  other_user_id uuid references user_ on update cascade on delete cascade not null,
  community_id uuid references community on update cascade on delete cascade not null,
  removed boolean default false,
  when_ timestamp not null default now()
);

-- When removed is false that means kicked
create table mod_add (
  id uuid NOT NULL DEFAULT next_uuid() primary key,
  mod_user_id uuid references user_ on update cascade on delete cascade not null,
  other_user_id uuid references user_ on update cascade on delete cascade not null,
  removed boolean default false,
  when_ timestamp not null default now()
);

