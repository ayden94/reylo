-- A sequence relation holds exactly one row. A view over one, a table of the same name in another
-- schema and a WITH query of its name are not it, and what it holds pins no key.
create sequence my_sequence;
create sequence called_seq;
select nextval('called_seq');
create table items (id bigint primary key, name text);
create view seq_view as select last_value from my_sequence;
create schema other;
create table other.my_sequence (last_value bigint);
