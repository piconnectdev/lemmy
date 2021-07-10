create table comment (
  id bigserial primary key,
  creator_id bigint references user_ on update cascade on delete cascade not null,
  post_id bigint references post on update cascade on delete cascade not null,
  parent_id bigint references comment on update cascade on delete cascade,
  content text not null,
  removed boolean default false not null,
  read boolean default false not null,
  published timestamp not null default now(),
  updated timestamp
);

create table comment_like (
  id bigserial primary key,
  user_id bigint references user_ on update cascade on delete cascade not null,
  comment_id bigint references comment on update cascade on delete cascade not null,
  post_id bigint references post on update cascade on delete cascade not null,
  score smallint not null, -- -1, or 1 for dislike, like, no row for no opinion
  published timestamp not null default now(),
  unique(comment_id, user_id)
);

create table comment_saved (
  id bigserial primary key,
  comment_id bigint references comment on update cascade on delete cascade not null,
  user_id bigint references user_ on update cascade on delete cascade not null,
  published timestamp not null default now(),
  unique(comment_id, user_id)
);
