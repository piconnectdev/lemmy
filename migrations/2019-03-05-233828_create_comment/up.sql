create table comment (
  id uuid NOT NULL DEFAULT next_uuid() primary key,
  creator_id uuid references user_ on update cascade on delete cascade not null,
  post_id uuid references post on update cascade on delete cascade not null,
  parent_id uuid references comment on update cascade on delete cascade,
  content text not null,
  removed boolean default false not null,
  read boolean default false not null,
  published timestamp not null default now(),
  updated timestamp
);

create table comment_like (
  id uuid NOT NULL DEFAULT next_uuid() primary key,
  user_id uuid references user_ on update cascade on delete cascade not null,
  comment_id uuid references comment on update cascade on delete cascade not null,
  post_id uuid references post on update cascade on delete cascade not null,
  score smallint not null, -- -1, or 1 for dislike, like, no row for no opinion
  published timestamp not null default now(),
  unique(comment_id, user_id)
);

create table comment_saved (
  id uuid NOT NULL DEFAULT next_uuid() primary key,
  comment_id uuid references comment on update cascade on delete cascade not null,
  user_id uuid references user_ on update cascade on delete cascade not null,
  published timestamp not null default now(),
  unique(comment_id, user_id)
);
