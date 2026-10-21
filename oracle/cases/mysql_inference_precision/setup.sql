-- The docs' schema, for two rules: a null-rejecting test in the WHERE proves the joined row and
-- the tested column, and JSON_ARRAYAGG is never NULL under a GROUP BY without ROLLUP. Users without
-- posts, posts without a date and a row of each kind are here, so a wrong non-null meets a NULL.
create table users (id int primary key, name varchar(40) not null, email varchar(80));
create table posts (id int primary key, user_id int, title varchar(80) not null, published_at datetime, key (user_id));
create table comments (id int primary key, post_id int not null, body varchar(80) not null);
create table events (id int primary key, kind varchar(10) not null check (kind in ('click', 'key')), payload json not null check (json_schema_valid('{"type": "object", "properties": {"x": {"type": "integer"}, "y": {"type": "integer"}, "code": {"type": "string"}}}', payload)), note varchar(10));
insert into users values (1, 'Ada', 'ada@example.com'), (2, 'Grace', null), (3, 'Linus', null);
insert into posts values (1, 1, 'Hello', '2026-01-01 00:00:00'), (2, 1, 'Draft', null), (3, 2, 'Notes', null);
insert into comments values (1, 1, 'First');
insert into events values (1, 'click', '{"x": 1, "y": 2}', null), (2, 'key', '{"code": "a"}', 'n'), (3, 'click', '{}', null);
