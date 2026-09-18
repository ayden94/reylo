-- Fixture for the lab: run once against the DATABASE_URL in .env.
-- psql "$DATABASE_URL" -f setup.sql
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
