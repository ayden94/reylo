-- Truth values (docs/driver-decoding.md, 2026-10-20). SQLite has no boolean type: a comparison, a
-- test or a logical operator computes 0, 1 or NULL, which is typed bool and which the oracle holds
-- to them; a column declared BOOLEAN or BOOL (as Prisma's schemas declare one) is typed bool too,
-- and holds whatever was stored in it (the seeds put text and other integers there), which the run
-- time refuses unless it is 0 or 1. An expression over such a column stays a number.
create table todos (id integer primary key, title text not null, done boolean not null default 0, starred bool, n integer, flag tinyint(1));
create view open_todos as select id, done, n > 1 as big from todos where not done;
