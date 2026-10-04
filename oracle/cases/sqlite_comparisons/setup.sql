-- A SQLite key pins a row only when the comparison leaves the key column as it is (the
-- soundness review of 2026-10-04): a numeric side converts a TEXT or untyped key ('01' and '1'
-- are both 1), a column on the left decides the collation, and a declared column named rowid
-- hides the rowid.
create table t (id integer primary key, code text unique);
insert into t values (1, '1'), (2, '01');
create table b (id integer primary key, n integer, nc text collate nocase);
create table a (id integer primary key, k unique, kt text unique);
insert into b values (1, 7, 'x');
insert into a values (1, 7, 'x'), (2, '7', 'X');
create table rt (rowid integer, v text);
insert into rt values (1, 'a'), (1, 'b');
