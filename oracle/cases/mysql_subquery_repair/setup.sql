-- A correlated ungrouped aggregate subquery in the select list makes the server flag every outer
-- column, and the subquery itself, nullable. The engine prepares the statement without its
-- subquery items and each subquery's item over its own FROM, and a column is NOT NULL when either
-- prepare says so: an outer column keeps its flag, and `count(*)` in a subquery that is exactly
-- one row is never NULL.
create table users (id int primary key, name varchar(20) not null, email varchar(40), score int not null default 0);
create table posts (id int primary key, user_id int not null, title varchar(20) not null, tag varchar(20), key (user_id));
