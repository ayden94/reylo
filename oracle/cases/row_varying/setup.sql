-- What reads no column of the FROM yet changes from row to row (the soundness review of
-- 2026-10-04): a window function, a system column, and a whole row read through a field, as a
-- DISTINCT value or as the value a joined key is equated to.
create table rv_posts (id int8 primary key, author_id int8, title text);
insert into rv_posts values (1, 1, 'a'), (2, 1, 'b'), (3, 2, 'c');
create table wr_users (id int4 primary key, email text unique);
create table wr_posts (id int4 primary key, author_id int4, title text);
insert into wr_users values (1, 'x'), (2, 'y');
insert into wr_posts values (1, 1, 'a'), (2, 2, 'b');
