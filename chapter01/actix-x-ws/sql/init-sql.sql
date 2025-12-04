CREATE TABLE r_posts (
  id SERIAL PRIMARY KEY,
  title VARCHAR NOT NULL,
  body TEXT NOT NULL,
  published BOOLEAN NOT NULL DEFAULT FALSE
)


alter table public.r_posts
    add created_at timestamp;

comment on column public.r_posts.created_at is '创建时间';

alter table public.r_posts
    add updated_at timestamp;

comment on column public.r_posts.updated_at is '更新时间';

alter table public.r_posts
    add created_by varchar(128);