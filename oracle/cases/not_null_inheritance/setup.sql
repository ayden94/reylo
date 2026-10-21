-- A NOT NULL set on an inheritance parent alone: a scan of the parent reads the child's rows too,
-- and RowDescription names the parent as their origin.
create table p2 (id int, x int);
create table c2 () inherits (p2);
alter table only p2 alter column x set not null;
create table plain (id int primary key, x int not null);
-- A partitioned table's NOT NULL holds in every partition.
create table pt (k int not null, x int not null) partition by list (k);
create table pt1 partition of pt for values in (1);
