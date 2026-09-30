-- Fixture for the lab: run once against the DATABASE_URL in .env.
-- psql "$DATABASE_URL" -f setup.sql
drop table if exists porg_posts;
drop table if exists porg_users;
drop type if exists porg_role;
create type porg_role as enum ('admin', 'member');
create table porg_users (
  id serial primary key,
  name text not null,
  email text,
  role porg_role not null default 'member',
  tags text[] not null default '{}',
  created_at timestamptz not null default now(),
  big bigint
);
insert into porg_users (name, email) values ('ada', 'ada@example.com'), ('bob', null);
create table porg_posts (
  id serial primary key,
  user_id int not null references porg_users(id),
  title text not null,
  body text,
  published_at timestamptz
);
insert into porg_posts (user_id, title, body) values (1, 'hello', null), (1, 'second', 'text');
