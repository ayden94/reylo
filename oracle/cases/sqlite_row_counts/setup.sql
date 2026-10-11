-- SQLite's row-count proof under a LIMIT, over a derived table and over joined one-row
-- subqueries: a LIMIT and an OFFSET only remove rows, and a FROM of one-row items is one row.
create table users (id integer primary key, name text not null, email text unique);
create table posts (id integer primary key, user_id integer not null references users(id), title text not null);
