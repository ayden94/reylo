-- SQLite keeps tables of its own in sqlite_schema beside the application's: the sequence table
-- of AUTOINCREMENT and the statistics of ANALYZE. They are tables a statement can read, and no
-- failure of the schema replay.
create table t (id integer primary key autoincrement, v text);
insert into t (v) values ('a'), ('b');
create index t_v on t (v);
analyze;
