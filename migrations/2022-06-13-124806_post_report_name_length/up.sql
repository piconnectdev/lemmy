-- adjust length limit to match post.name
alter table post_report alter column original_post_name type varchar(1000);
