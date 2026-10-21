-- SQLite folds ASCII case in a name and nothing else: `books`, `BOOKS` and "Books" are one table
-- and `ID` is its column `id`, while `été` is no spelling of `Été`. A declared column named
-- rowid, in any case, hides the rowid.
create table Books (id integer primary key, Title text not null, ISBN text unique, note text);
insert into Books values (1, 'a', 'x', null), (2, 'b', 'y', 'n');
create table "Été" (id integer primary key, v text);
create table rt (RowId integer, v text);
insert into rt values (1, 'a'), (1, 'b');
create table wr (K text primary key, v text) without rowid;
create table "my table" (id integer primary key, v text);
create trigger "skip one" before insert on "MY TABLE" begin select raise(ignore) where new.v = 'a'; end;
