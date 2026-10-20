drop table if exists posts;
drop table if exists users;
drop type if exists user_role;
create type user_role as enum ('admin', 'member');
create table users (
  id serial primary key,
  name text not null,
  email text,
  role user_role not null default 'member',
  created_at timestamptz not null default now()
);
create table posts (
  id serial primary key,
  user_id int not null references users(id),
  title text not null,
  body text
);
insert into users (name, email, role) values ('ada', 'ada@example.com', 'admin'), ('bob', null, 'member');
insert into posts (user_id, title, body) values (1, 'Hello', 'first post'), (1, 'Again', null);
