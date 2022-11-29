create table site_language (
  id uuid NOT NULL DEFAULT next_uuid() primary key,
  site_id uuid references site on update cascade on delete cascade not null,
  language_id int references language on update cascade on delete cascade not null,
  unique (site_id, language_id)
);

create table community_language (
  id uuid NOT NULL DEFAULT next_uuid() primary key,
  community_id uuid references community on update cascade on delete cascade not null,
  language_id int references language on update cascade on delete cascade not null,
  unique (community_id, language_id)
);

-- update existing users, sites and communities to have all languages enabled
do $$
    declare
        xid uuid;
begin
    for xid in select id from local_user
    loop
        insert into local_user_language (local_user_id, language_id)
        (select xid, language.id as lid from language);
    end loop;

    for xid in select id from site
    loop
        insert into site_language (site_id, language_id)
        (select xid, language.id as lid from language);
    end loop;

    for xid in select id from community
    loop
        insert into community_language (community_id, language_id)
        (select xid, language.id as lid from language);
    end loop;
end;
$$;
