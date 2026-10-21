-- MySQL's prepare says nothing of a parameter's nullability, so the engine reads it off the
-- statement by the rules of Postgres and SQLite: a parameter written whole to a column that may
-- hold NULL (a VALUES row, INSERT's SET, an UPDATE's SET, ON DUPLICATE KEY UPDATE, a REPLACE)
-- takes NULL, unless it also fills a NOT NULL column, and so does one the statement tests for
-- NULL (IS NULL, IS NOT NULL, ISNULL(), <=>) or passes first to COALESCE or IFNULL, directly or
-- through the column of a WITH query or a derived table that selects it. The oracle binds NULL for
-- each such parameter beside its other values. The INVISIBLE column takes no value from an INSERT
-- without a column list.
create table users (id int primary key auto_increment, name varchar(20) not null, secret varchar(20) not null default 's' invisible, email varchar(40), bio varchar(40), unique key users_email (email));
create table posts (id int primary key auto_increment, user_id int not null, title varchar(40) not null, tag varchar(20));
