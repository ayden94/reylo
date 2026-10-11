-- A name qualified with a schema is that schema's: a stored function there is no built-in and no
-- aggregate, whatever it is called, and a table there has keys of its own. `reylo_oracle_app.users`
-- has no key and holds the same `id` twice; its `count` returns its argument, and its
-- `json_object` returns NULL. In this schema a stored function `count` is reached by a name in
-- backticks; a space before the parenthesis reaches it only without IGNORE_SPACE, which mysql2's
-- session has (mysql_no_ignore_space has the other).
drop database if exists reylo_oracle_app;
create database reylo_oracle_app;
create table reylo_oracle_app.users (id bigint not null, email varchar(40));
insert into reylo_oracle_app.users values (1, 'a'), (1, 'b'), (2, 'a');
create function reylo_oracle_app.count(x bigint) returns bigint deterministic return x;
create function reylo_oracle_app.json_object(k varchar(10), v bigint) returns json deterministic return null;
create function reylo_oracle_app.json_arrayagg(v bigint) returns json deterministic return null;
create function reylo_oracle_app.lower(s varchar(40)) returns varchar(40) not deterministic no sql return elt(1 + floor(rand() * 3), 'a', 'b', '1');
create function reylo_oracle_app.coalesce(a json, b json) returns json deterministic return null;
create table users (id bigint primary key, email varchar(40) unique, name varchar(20) not null);
create table posts (id bigint primary key, user_id bigint not null, title varchar(20) not null);
create function `count`(x bigint) returns bigint deterministic return x;
create function `max`(x bigint) returns bigint deterministic return x;
