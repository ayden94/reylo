-- Statements that do not end over the seeded rows: a series from an infinite timestamp, and a
-- recursive walk over rows that name each other, as every seeded node does through node 1, its
-- own parent. The oracle bounds every run (the timeout file here, in milliseconds), reports a
-- run it stopped as timed out, and takes no evidence from it: forever.sql never runs
-- (unchecked.txt), hours.sql runs only over the plain state, and ancestors.sql only from the
-- two nodes below, which are on no cycle.
create table events (id int primary key, at timestamptz not null);
create table nodes (id int primary key, parent_id int, name text);
insert into nodes values (0, null, 'root'), (2, 0, null);
