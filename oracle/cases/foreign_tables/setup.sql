-- PostgreSQL enforces no constraint on a foreign table's rows (the soundness review of
-- 2026-10-04): a foreign table's NOT NULL and CHECK, and a partitioned table with a foreign
-- partition, prove nothing; a local table's do.
create extension file_fdw;
create server fs foreign data wrapper file_fdw;
create foreign table ft (id int4 not null, status text check (status in ('a', 'b'))) server fs options (program 'printf "1\tzzz\n\\\\N\tb\n"');
create table pt (id int4 not null, status text check (status in ('a', 'b'))) partition by list (status);
create table pt_local partition of pt for values in ('a');
create foreign table pt_remote partition of pt for values in ('b') server fs options (program 'printf "\\\\N\tzzz\n"');
create table lt (id int4 not null, status text check (status in ('a', 'b')));
insert into lt values (1, 'a');
