alter table community add column deleted boolean default false not null;
alter table post add column deleted boolean default false not null;
alter table comment add column deleted boolean default false not null;

-- The views
drop view community_view;

create view community_view as 
with all_community as
(
  select *,
  (select name from user_ u where c.creator_id = u.id) as creator_name,
  (select name from category ct where c.category_id = ct.id) as category_name,
  (select count(*) from community_follower cf where cf.community_id = c.id) as number_of_subscribers,
  (select count(*) from post p where p.community_id = c.id) as number_of_posts,
  (select count(*) from comment co, post p where c.id = p.community_id and p.id = co.post_id) as number_of_comments
  from community c
)

select
ac.*,
u.id as user_id,
(select cf.id::integer::boolean from community_follower cf where u.id = cf.user_id and ac.id = cf.community_id) as subscribed
from user_ u
cross join all_community ac

union all

select 
ac.*,
null as user_id,
null as subscribed
from all_community ac
;


drop view post_view;
create view post_view as
with all_post as
(
  select        
  p.*,
  (select name from user_ where p.creator_id = user_.id) as creator_name,
  (select name from community where p.community_id = community.id) as community_name,
  (select removed from community c where p.community_id = c.id) as community_removed,
  (select deleted from community c where p.community_id = c.id) as community_deleted,
  (select count(*) from comment where comment.post_id = p.id) as number_of_comments,
  coalesce(sum(pl.score), 0) as score,
  count (case when pl.score = 1 then 1 else null end) as upvotes,
  count (case when pl.score = -1 then 1 else null end) as downvotes,
  hot_rank(coalesce(sum(pl.score) , 0), p.published) as hot_rank
  from post p
  left join post_like pl on p.id = pl.post_id
  group by p.id
)

select
ap.*,
u.id as user_id,
coalesce(pl.score, 0) as my_vote,
(select cf.id::integer from community_follower cf where u.id = cf.user_id and cf.community_id = ap.community_id)::bool as subscribed,
(select pr.id::integer from post_read pr where u.id = pr.user_id and pr.post_id = ap.id)::bool as read,
(select ps.id::integer from post_saved ps where u.id = ps.user_id and ps.post_id = ap.id)::bool as saved
from user_ u
cross join all_post ap
left join post_like pl on u.id = pl.user_id and ap.id = pl.post_id

union all

select 
ap.*,
null as user_id,
null as my_vote,
null as subscribed,
null as read,
null as saved
from all_post ap
;

drop view reply_view;
drop view comment_view;
create view comment_view as
with all_comment as
(
  select        
  c.*,
  (select community_id from post p where p.id = c.post_id),
  (select u.banned from user_ u where c.creator_id = u.id) as banned,
  (select cb.id::bool from community_user_ban cb, post p where c.creator_id = cb.user_id and p.id = c.post_id and p.community_id = cb.community_id) as banned_from_community,
  (select name from user_ where c.creator_id = user_.id) as creator_name,
  coalesce(sum(cl.score), 0) as score,
  count (case when cl.score = 1 then 1 else null end) as upvotes,
  count (case when cl.score = -1 then 1 else null end) as downvotes
  from comment c
  left join comment_like cl on c.id = cl.comment_id
  group by c.id
)

select
ac.*,
u.id as user_id,
coalesce(cl.score, 0) as my_vote,
(select cs.id::bool from comment_saved cs where u.id = cs.user_id and cs.comment_id = ac.id) as saved
from user_ u
cross join all_comment ac
left join comment_like cl on u.id = cl.user_id and ac.id = cl.comment_id

union all

select 
    ac.*,
    null as user_id, 
    null as my_vote,
    null as saved
from all_comment ac
;

create view reply_view as 
with closereply as (
    select 
    c2.id, 
    c2.creator_id as sender_id, 
    c.creator_id as recipient_id
    from comment c
    inner join comment c2 on c.id = c2.parent_id
    where c2.creator_id != c.creator_id
    -- Do union where post is null
    union
    select
    c.id,
    c.creator_id as sender_id,
    p.creator_id as recipient_id
    from comment c, post p
    where c.post_id = p.id and c.parent_id is null and c.creator_id != p.creator_id
)
select cv.*,
closereply.recipient_id
from comment_view cv, closereply
where closereply.id = cv.id
;

