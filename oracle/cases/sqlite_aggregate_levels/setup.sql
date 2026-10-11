-- An aggregate belongs to the innermost query one of the columns it reads comes from, and a bare
-- column name comes from the innermost query whose FROM has it, which SQLite resolves and marks
-- on the SELECT. Over columns of an outer query only, the scalar subquery around the call is a
-- row per row of its own FROM: NULL where that is empty. `name` is a column of `users` alone,
-- `body` of `comments` alone, `title` of all three.
create table users (id integer primary key, name text not null, title text);
create table posts (id integer primary key, user_id integer not null references users (id), title text not null);
create table comments (id integer primary key, post_id integer not null, body text not null, title text);
