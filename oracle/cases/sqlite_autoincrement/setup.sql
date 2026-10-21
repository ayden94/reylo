-- An AUTOINCREMENT key at the largest integer leaves a keyless INSERT no rowid to take: SQLite
-- answers SQLITE_FULL and, for an INSERT of one row, rolls the whole transaction back itself,
-- the seeded rows with it. The seeding keeps the largest integer out of such a key (notes), and
-- seeds again when a statement's error ended the transaction, as a_full.sql's does: its table
-- holds the last rowid before any seeding, so it never runs (unchecked.txt), and the statements
-- after it are still checked.
create table notes (id integer primary key autoincrement, body text not null, pinned integer);
create table tags (id integer primary key, name text not null);
create table full_log (id integer primary key autoincrement, note text);
insert into full_log values (9223372036854775807, 'the last rowid');
