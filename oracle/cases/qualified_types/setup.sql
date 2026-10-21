-- A type in a schema outside search_path has to be named with its schema: the engine's
-- type_name is the bare catalog name, so a parameter is cast to the type the server gave the
-- placeholder, by oid, and never to that name.
create schema app;
create type app.status as enum ('open', 'closed');
create domain app.code as text check (value ~ '^[a-z]+$');
create table app.tickets (id int primary key, status app.status not null, code app.code, title text);
-- The same name in search_path, another type: a cast by the bare name would pick this one.
create type status as enum ('up', 'down');
