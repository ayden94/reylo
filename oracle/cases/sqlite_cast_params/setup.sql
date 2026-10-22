-- A parameter under the cast a `:name::type` compiles to on SQLite, `CAST(? AS type)` or `json(?)`
-- for `::json`, takes NULL wherever the bare parameter does: written whole to a nullable column,
-- coalesced or tested for NULL. Into a NOT NULL column it is a value.
create table users (id integer primary key, name text not null, email text unique, score real, n integer, doc text, meta text not null default '{}') strict;
insert into users (name, email, score, n, doc) values ('Ada', 'ada@example.com', 1.5, 1, '{"a":1}');
