-- MERGE … RETURNING (PostgreSQL 17): the target's row is the one the action wrote or removed, and
-- the source's is no row at all where the MERGE acts on a target row the source does not match.
create table users (id int primary key, name text not null, nick text);
create table other (id int primary key, name text not null, label text not null);
insert into users values (1, 'a', null), (2, 'b', null), (3, 'c', null);
insert into other values (1, 'x', 'l1'), (4, 'y', 'l4');
