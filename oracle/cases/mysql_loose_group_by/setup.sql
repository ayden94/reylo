-- Without ONLY_FULL_GROUP_BY the server takes a column beside an aggregate: the value is any row's,
-- and NULL where no row matched. And the engine's probe of a JSON constructor, the value
-- expressions as columns of their own statement, prepares where an aggregate stands beside a
-- column, so the fields of such a document are typed: beside an aggregate the server writes a
-- truth value as 1 or 0.
create table users (id int primary key, name varchar(20) not null, big bigint, flag boolean not null default true);
create table posts (id int primary key, user_id int not null, title varchar(20) not null, key (user_id));
