-- A row value compares element by element, so each parameter in one takes the type of the element
-- it is compared with: in =, <>, <, <=, >, >=, IS and IS NOT, BETWEEN, IN with a subquery or a list
-- of row values (which SQLite reads as VALUES), a CASE whose base is a row value, a row subquery,
-- and UPDATE's SET (a, b) = (SELECT ...). A keyset page is the common one.
create table users (id integer primary key, name text not null, created_at text not null) strict;
create table events (id integer primary key, at text not null, n integer);
