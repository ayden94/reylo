-- PostgreSQL 18 lets a not-null constraint be added NOT VALID: new rows are checked, rows already
-- there are not, and pg_attribute.attnotnull is true all the same.
create table legacy (id int primary key, x int, y int not null);
insert into legacy values (1, null, 1);
alter table legacy add constraint legacy_x_nn not null x not valid;
