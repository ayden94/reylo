-- An aggregate belongs to the innermost query one of the columns it reads comes from, and a bare
-- column name comes from the innermost query whose FROM has it. Over columns of an outer query
-- only, the scalar subquery around the call is a row per row of its own FROM: NULL where that is
-- empty. `name` is a column of `users` alone, `body` of `comments` alone, `title` of all but
-- `comments`, and `drafts` never holds a row.
create table users (id bigint primary key, name varchar(20) not null, title varchar(20));
create table posts (id bigint primary key, user_id bigint not null, title varchar(20) not null);
create table comments (id bigint primary key, post_id bigint not null, body varchar(20) not null);
create table drafts (id bigint primary key, title varchar(20) not null, check (false));
create view post_titles as select id, title from posts;
