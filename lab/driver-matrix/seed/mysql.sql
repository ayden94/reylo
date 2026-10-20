-- The fixture every MySQL-family row reads, in the driver_matrix database of each container.
create database if not exists driver_matrix;
use driver_matrix;
drop table if exists people, scratch, uniq, typed;
create table people (id int primary key, name varchar(50) not null, active boolean not null, score int null);
insert into people values (1, 'ada', true, 10), (2, 'bob', false, null), (3, 'cy', true, 30);
create table scratch (id int auto_increment primary key, tag varchar(200) not null, n int);
create table uniq (k varchar(50) primary key);
insert into uniq values ('taken');
-- One value of each type whose JavaScript representation drivers disagree on (S10).
create table typed (id int primary key, big bigint, ubig bigint unsigned, amount decimal(30,18), day date, at datetime(3), doc json, bit1 bit(1), raw varbinary(4), tags set('x','y','z'), kind enum('a','b'), flag tinyint(1), yr year, f8 double, tm time, s text);
insert into typed values (1, -9007199254740993, 18446744073709551615, 1.100000000000000001, '2026-10-20', '2026-10-20 10:00:00.123', '{"a": [1, 2]}', b'1', x'0102ff', 'x,z', 'b', 1, 2026, 2.5, '10:00:00', 'hi');
