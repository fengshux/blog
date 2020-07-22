-- CREATE database 
CREATE DATABASE blog;


CREATE TABLE post (
   id bigserial PRIMARY KEY  NOT NULL,
   title VARCHAR(128) NOT NULL,
   body text,
   create_time timestamptz NOT NULL DEFAULT now(),
   update_time timestamptz NOT NULL DEFAULT now()
);

comment ON TABLE post  IS '文章';
comment ON COLUMN post.id IS '主键';
comment ON COLUMN post.title IS '文章内容';
comment ON COLUMN post.body IS '文章内容';
comment ON COLUMN post.create_time IS '创建时间';
comment ON COLUMN post.update_time IS '更新时间';


-- test sql
insert into post (title, body) values ('The first blog', 'blooming.');
