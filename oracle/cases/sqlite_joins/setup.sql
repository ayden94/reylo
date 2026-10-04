-- SQLite's row-count proof over joins, grouping, DISTINCT and set operations, and the ON terms
-- SQLite moves into the WHERE.
create table users (id integer primary key, name text not null, email text unique);
create table posts (id integer primary key, user_id integer not null references users(id), title text not null);
