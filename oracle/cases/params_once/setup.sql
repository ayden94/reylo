-- Each parameter appears once in a statement, so that one place types it (the maintainer's rule of
-- 2026-10-11): a second use of a name, `:name` or `$n`, is an error at its place, and a value the
-- statement reads twice is selected once in a WITH. A test of the WITH query's column for NULL, or
-- a coalesce of it, lets the caller pass NULL as the same test of the parameter did, and the
-- column pins a key as the parameter does.
create table users (id int primary key, name text not null, email text, age int);
