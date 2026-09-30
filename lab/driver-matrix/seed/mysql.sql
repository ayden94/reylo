-- The fixture every MySQL-family row reads, in the driver_matrix database of each container.
create database if not exists driver_matrix;
use driver_matrix;
drop table if exists people, scratch, uniq;
create table people (id int primary key, name varchar(50) not null, active boolean not null, score int null);
insert into people values (1, 'ada', true, 10), (2, 'bob', false, null), (3, 'cy', true, 30);
create table scratch (id int auto_increment primary key, tag varchar(200) not null, n int);
create table uniq (k varchar(50) primary key);
insert into uniq values ('taken');
