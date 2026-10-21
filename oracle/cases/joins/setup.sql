-- Joins on unique keys, chains, self-joins, derived tables, WITH queries and LATERAL; DISTINCT
-- and GROUP BY over pinned values; INTERSECT and EXCEPT; and the shapes that stay any number
-- of rows. Keys are bigint, so a value of unknown type does not pin them.
create table users (id bigint primary key, email text unique, name text, pinned_post bigint);
create table posts (id bigint primary key, author_id bigint references users (id), title text, score float8);
create table comments (id bigint primary key, post_id bigint references posts (id), body text);
create table profiles (user_id bigint primary key references users (id), bio text);
