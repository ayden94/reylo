-- PostgreSQL 18's syntax, as the parser of 18 reads it: RETURNING WITH (OLD AS …, NEW AS …) and
-- the names it hides or that hide it, a WITHOUT OVERLAPS key, a virtual generated column, a CHECK
-- that is NOT ENFORCED, and a jsonb null, which 18 casts to an SQL NULL where 17 raised an error.
create extension btree_gist;
create table users (id int primary key, name text not null, nick text, o text);
create table other (id int primary key, name text not null);
insert into other values (1, 'x'), (2, 'y');
-- A table and a column that go by the names RETURNING gives the old and the new row.
create table old (id int primary key, name text not null);
create table t2 (id int primary key, n text not null);
-- A temporal key: two bookings of one room stand side by side, so the room alone names no row.
create table bookings (room int, during int4range, who text not null, primary key (room, during without overlaps));
insert into bookings values (1, '[1,3)', 'a'), (1, '[3,5)', 'b'), (2, '[1,3)', 'c');
create table slots (room int not null, during int4range not null, note text, unique (room, during without overlaps));
insert into slots values (1, '[1,3)', 'early'), (1, '[7,9)', 'late');
create table stays (room int, during int4range, guest text, foreign key (room, period during) references bookings (room, period during));
insert into stays values (1, '[1,4)', 'g');
-- Virtual generated columns: computed when read, NOT NULL and CHECK checked when the row is written.
create table g (
  id int primary key, a int not null, b int,
  v int generated always as (a * 2) virtual not null,
  w int generated always as (b * 2),
  s int generated always as (a + 1) stored not null,
  k text generated always as (case when a > 0 then 'pos' else 'neg' end) virtual not null check (k in ('pos', 'neg'))
);
-- A CHECK the server does not enforce says nothing of the rows.
create table checked (id int primary key, status text not null, constraint checked_status check (status in ('a', 'b')) not enforced);
insert into checked values (1, 'zzz');
create table docs (id int primary key, j jsonb not null);
