-- The fixture every SQLite row reads: each job gets a fresh database file (or D1 instance) seeded with this.
drop table if exists people;
drop table if exists scratch;
drop table if exists uniq;
create table people (id integer primary key, name text not null, active integer not null, score integer);
insert into people values (1, 'ada', 1, 10), (2, 'bob', 0, null), (3, 'cy', 1, 30);
create table scratch (id integer primary key autoincrement, tag text not null, n integer);
create table uniq (k text primary key);
insert into uniq values ('taken');
