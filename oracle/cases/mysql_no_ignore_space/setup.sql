-- Without IGNORE_SPACE (the server's default `sql_mode`, which a session set by the project would
-- have; mysql2's and mariadb's client flags add IGNORE_SPACE) a built-in's name with a space before
-- its parenthesis is a stored function's: `count (id)` calls the `count` of this schema, which
-- returns its argument, once per row.
create table users (id bigint primary key, email varchar(40) unique, name varchar(20) not null);
create table posts (id bigint primary key, user_id bigint not null, title varchar(20) not null);
create function `count`(x bigint) returns bigint deterministic return x;
create function `max`(x bigint) returns bigint deterministic return x;
