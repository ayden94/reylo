-- What a WHERE or a CASE proves non-null, and where that proof stops (the soundness review of
-- 2026-10-04): a column merged by USING, one occurrence of a table that appears twice, a
-- subquery named like a table, and a window function that reads other rows than the one a WHEN
-- tests.
create table ja (x int4, v text);
create table jb (x int4, w text);
insert into ja values (1, 'a'), (7, 'b');
insert into jb values (1, 'w'), (8, 'z');
create table people (id int8 primary key, name text not null, boss int8);
insert into people values (1, 'a', 99), (2, 'b', 1);
create table authors (id int8 primary key, name text not null);
create table books (id int8 primary key, author_id int8);
insert into authors values (1, 'a');
insert into books values (1, 1), (2, 99);
create table w (id int4 primary key, x int4);
insert into w values (1, null), (2, 5), (3, 6);
