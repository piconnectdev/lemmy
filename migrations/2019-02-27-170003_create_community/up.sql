create table category (
  id serial primary key,
  name varchar(100) not null unique
);

insert into category (name) values
('Discussion'),
('Humor/Memes'),
('Gaming'),
('Movies'),
('TV'),
('Music'),
('Literature'),
('Comics'),
('Photography'),
('Art'),
('Learning'),
('DIY'),
('Lifestyle'),
('News'),
('Politics'),
('Society'),
('Gender/Identity/Sexuality'),
('Race/Colonisation'),
('Religion'),
('Science/Technology'),
('Programming/Software'),
('Health/Sports/Fitness'),
('Porn'),
('Places'),
('Meta'),
('Other');

create table community (
  id uuid NOT NULL DEFAULT next_uuid() primary key,
  name varchar(20) not null unique,
  title varchar(100) not null,
  description text,
  category_id int references category on update cascade on delete cascade not null,
  creator_id uuid references user_ on update cascade on delete cascade not null,
  removed boolean default false not null,
  published timestamp not null default now(),
  updated timestamp
);

create table community_moderator (
  id uuid NOT NULL DEFAULT next_uuid() primary key,
  community_id uuid references community on update cascade on delete cascade not null,
  user_id uuid references user_ on update cascade on delete cascade not null,
  published timestamp not null default now(),
  unique (community_id, user_id)
);

create table community_follower (
  id uuid NOT NULL DEFAULT next_uuid() primary key,
  community_id uuid references community on update cascade on delete cascade not null,
  user_id uuid references user_ on update cascade on delete cascade not null,
  published timestamp not null default now(),
  unique (community_id, user_id)
);

create table community_user_ban (
  id uuid NOT NULL DEFAULT next_uuid() primary key,
  community_id uuid references community on update cascade on delete cascade not null,
  user_id uuid references user_ on update cascade on delete cascade not null,
  published timestamp not null default now(),
  unique (community_id, user_id)
);

insert into community (name, title, category_id, creator_id) values ('main', 'The Default Community', 1, '00000000-0000-0000-0000-000000000001');

create table site (
  id bigserial primary key,
  name varchar(20) not null unique,
  description text,
  creator_id uuid references user_ on update cascade on delete cascade not null,
  published timestamp not null default now(),
  updated timestamp
);
