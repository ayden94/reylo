-- A parameter written whole to a column, bare or under a cast, takes NULL where the column does,
-- an element of a row value `SET (a, b) = (:a, :b)` included. Into a NOT NULL column it is a value.
create table users (id int primary key, name text not null, email text unique, score numeric, n int, doc jsonb, meta json not null default '{}');
insert into users (id, name, email) values (1, 'Ada', 'ada@example.com');
