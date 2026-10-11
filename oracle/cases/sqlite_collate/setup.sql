-- A COLLATE names how a value compares and leaves the value as it is: a parameter under one
-- keeps the type of what it is compared with. The key is another matter: `b` is unique under
-- BINARY, which tells 'a' from 'A', and a comparison under NOCASE meets both rows.
create table c (id integer primary key, b text unique, n text unique collate nocase, name text not null);
insert into c values (1, 'a', 'x', 'p'), (2, 'A', 'y', 'P');
