-- What an INSERT's source says of its parameters: every row of a VALUES list writes to the target
-- columns, as the one-row form does, and so does the list of the SELECT that feeds an INSERT.
create table users (id integer primary key, name text not null, email text unique, nick text);
create table bar (name text not null, ready bool not null, n integer);
insert into bar values ('a', 1, 1), ('b', 0, 2), ('c', 'yes', 3);
create table foo (name text not null, meta text, k integer);
create table foo2 (name text not null, meta text, k integer);
