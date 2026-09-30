-- The fixture every Postgres row reads: the local server's driver_matrix schema, the Neon
-- proxy's container, and each PGlite instance. Re-running it resets the three tables.
create schema if not exists driver_matrix;
drop table if exists driver_matrix.people, driver_matrix.scratch, driver_matrix.uniq;
create table driver_matrix.people (id int primary key, name text not null, active boolean not null, score int);
insert into driver_matrix.people values (1, 'ada', true, 10), (2, 'bob', false, null), (3, 'cy', true, 30);
create table driver_matrix.scratch (id serial primary key, tag text not null, n int);
create table driver_matrix.uniq (k text primary key);
insert into driver_matrix.uniq values ('taken');
