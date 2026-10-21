-- An aggregate belongs to the innermost query one of the columns it reads comes from, and a bare
-- column name comes from the innermost query whose FROM has it. Over columns of an outer query
-- only, the scalar subquery around the call is a row per row of its own FROM: NULL where that is
-- empty. `name` is a column of `users` alone, `body` of `comments` alone, `title` of all but
-- `comments`, and `drafts` never holds a row.
create table users (id bigint primary key, name text not null, title text);
create table posts (id bigint primary key, user_id bigint not null references users (id), title text not null);
create table comments (id bigint primary key, post_id bigint not null, body text not null);
create table drafts (id bigint primary key, title text not null, check (false));
create function pair(out a int, out b text) language sql as $$ select 1, 'x' where false $$;
