-- A parameter written whole and bare to a column takes NULL where the column does, an element of a
-- row value `SET (a, b) = (:a, :b)` included; into a NOT NULL column it is a value. Under a cast
-- it is the cast's type and takes no NULL; until 2026-10-21 it took NULL
-- where the bare one did.
create table users (id int primary key, name text not null, email text unique, score numeric, n int, doc jsonb, meta json not null default '{}');
insert into users (id, name, email) values (1, 'Ada', 'ada@example.com');
