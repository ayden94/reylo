-- A JSON constructor's shape, read from the server's types of its value expressions: an object
-- from `json_object`, a list from `json_arrayagg`, never NULL only where nothing can make it so.
-- Inside a document the server writes a truth value as `true` or `false`, a DECIMAL as a number,
-- a date as a string and a binary value as a string.
create table users (id int primary key, name varchar(20) not null, email varchar(40), flag boolean not null default true, score decimal(10,2), ratio double, born date, seen datetime, at time, doc json, raw varbinary(8), kind enum('a','b'), bits bit(8), y year, big bigint, tags set('x','y'));
create table posts (id int primary key, user_id int not null, title varchar(20) not null, tag varchar(20), key (user_id));
create table empty_posts (id int primary key, user_id int not null, title varchar(20) not null, check (false));
