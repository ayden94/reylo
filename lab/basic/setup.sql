-- Fixture for the lab: run once against the DATABASE_URL in .env.
-- psql "$DATABASE_URL" -f setup.sql
drop table if exists lando_posts;
drop table if exists lando_users;
drop type if exists lando_role;
create type lando_role as enum ('admin', 'member');
create table lando_users (
  id serial primary key,
  name text not null,
  email text,
  role lando_role not null default 'member',
  tags text[] not null default '{}',
  created_at timestamptz not null default now(),
  big bigint
);
insert into lando_users (name, email) values ('ada', 'ada@example.com'), ('bob', null);
create table lando_posts (
  id serial primary key,
  user_id int not null references lando_users(id),
  title text not null,
  body text,
  published_at timestamptz
);
insert into lando_posts (user_id, title, body) values (1, 'hello', null), (1, 'second', 'text');
