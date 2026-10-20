-- The fixture every SQLite row reads: each job gets a fresh database file (or D1 instance) seeded with this.
drop table if exists people;
drop table if exists scratch;
drop table if exists uniq;
drop table if exists typed;
create table people (id integer primary key, name text not null, active integer not null, score integer);
insert into people values (1, 'ada', 1, 10), (2, 'bob', 0, null), (3, 'cy', 1, 30);
create table scratch (id integer primary key autoincrement, tag text not null, n integer);
create table uniq (k text primary key);
insert into uniq values ('taken');
-- One value of each storage class, JSON text and a truth value (S10), and an integer past 2^53, which Reylo refuses.
create table typed (id integer primary key, n integer, r real, s text, b blob, doc text, flag integer, x, big integer);
insert into typed values (1, 5, 1.5, 'hi', x'0102ff', '{"a":[1,2]}', 1, 7, 9007199254740993);
