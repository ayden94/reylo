-- Statements that do not end: a recursive count with no bound, alone and inside a write, and one
-- whose bound is a parameter. The oracle interrupts a run at the timeout (the timeout file here,
-- in milliseconds), reports it as timed out and takes no evidence from it: forever.sql and
-- forever_write.sql never run (unchecked.txt), and bounded.sql runs under a bound it reaches.
-- SQLite rolls the whole transaction back when it interrupts a write, so the oracle seeds again
-- and z_counters.sql, which runs after it, still reads the seeded rows.
create table counters (id integer primary key, n integer, label text);
