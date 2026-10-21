-- What bounds a SELECT's rows besides a key: an aggregate with no GROUP BY is one row, a
-- statement with no FROM is one, one group where the WHERE pins a key and every GROUP BY column
-- is that relation's, and nothing else.
create table users (id int primary key, email varchar(40) not null unique, name varchar(20) not null, age int);
create table posts (id int primary key, user_id int not null, title varchar(20) not null, key (user_id));
create table tags (post_id int not null, tag varchar(20) not null, primary key (post_id, tag));
create view user_emails as select id, email from users;
create table parts (id int primary key, v int) partition by hash (id) partitions 2;
