-- The functions a statement may call are those of the SQLite that runs it. This oracle runs
-- rusqlite's bundled SQLite, which has soundex and no math functions; the engine's own has both.
create table users (id integer primary key, name text not null, email text, score real, doc text);
insert into users values (1, 'Robert', null, 0.5, '{"a": 1, "b": "x"}'), (2, 'Rupert', 'r@example.com', null, '[1, 2]');
create view shouting as select id, upper(name) as name, soundex(name) as sound from users;
