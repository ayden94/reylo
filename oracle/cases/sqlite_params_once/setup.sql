-- Each parameter appears once in a statement, so that one place types it:
-- a second use of a name, `:name`, `@name`, `$name` or `?n` (a bare `?` takes the
-- next number, so `?1` after it names it again), is an error at its place, and a value the
-- statement reads twice is selected once in a WITH. A coalesce of the WITH query's column, however
-- the statement names it, lets the caller pass NULL as a coalesce of the parameter does; a test of
-- it for NULL does not, as a test of the parameter does not; and the
-- column pins a key as the parameter does.
create table users (id integer primary key, name text not null, email text, age integer);
