create table custom_emoji (
  id uuid NOT NULL DEFAULT next_uuid() primary key,
  local_site_id uuid references local_site on update cascade on delete cascade not null,
  shortcode varchar(128) not null UNIQUE,
  image_url text not null UNIQUE,
  alt_text text not null,
  category text not null,
  published timestamp without time zone default now() not null,
  updated timestamp without time zone
);

create table custom_emoji_keyword (
  id uuid NOT NULL DEFAULT next_uuid() primary key,
  custom_emoji_id uuid references custom_emoji on update cascade on delete cascade not null,
  keyword varchar(128) not null,
  UNIQUE (custom_emoji_id, keyword)
);

create index idx_custom_emoji_category on custom_emoji (id,category);
