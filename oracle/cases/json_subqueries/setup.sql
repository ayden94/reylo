-- A JSON constructor's shape where a name or a subquery decides it. A bare name is a column
-- before it is a FROM item, of the innermost query that has one of that name, so `to_jsonb(u)` is
-- a whole row only where no FROM in scope has a column `u`: `users` has one, and a column `p`.
-- And a scalar subquery that may find no row is NULL whatever its value is: a column, a whole
-- row, or an aggregate of the query around it.
create table users (id bigint primary key, name text not null, u jsonb, p text);
create table posts (id bigint primary key, user_id bigint not null, title text not null);
