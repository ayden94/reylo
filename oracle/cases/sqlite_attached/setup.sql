-- A database the application attaches is on its connection, not in the file: the type check
-- reads the file, so a table in the attached database cannot be typed, and the error says so.
create table users (id integer primary key, name text not null);
attach ':memory:' as baz;
create table baz.users (id integer primary key, name text not null);
create table baz.notes (id integer primary key, body text);
