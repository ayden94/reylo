-- A user-defined = that is true for a NULL operand: a WHERE that tests the outer-joined table with
-- it does not prove that the joined row exists.
create table users (id int primary key, name text not null, email text not null);
create table posts (id int primary key, editor_id int references users (id), title text not null);
create function eq_null_true(text, int) returns bool language sql immutable as $$ select $1 is null or length($1) = $2 $$;
create operator = (leftarg = text, rightarg = int, function = eq_null_true);
