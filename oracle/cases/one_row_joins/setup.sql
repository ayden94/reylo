-- A cross join of items of exactly one row is one row, and so is an outer join whose preserved
-- side is one row and whose other side is one at most. An inner join under a condition, a WHERE
-- and a FULL JOIN are not.
create table bar (id bigint primary key, name text);
create table users (id bigint primary key, email text unique, name text);
create sequence seq;
