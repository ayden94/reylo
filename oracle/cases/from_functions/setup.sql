-- A function in FROM that returns no set is exactly one row: its value, a row of NULLs for a
-- NULL composite, or an error. A set-returning one, by any overload of its name, is any number.
-- Its one column is as nullable as the same call in a select list, and an outer join nulls it.
create table users (id bigint primary key, email text unique not null, name text);
create type one_field as (a int);
create function one_text(p text) returns text language sql as $$ select p $$;
create function out_pair(p int, out a int, out b text) language sql as $$ select p, p::text $$;
create function strict_pair(p int, out a int, out b text) strict language sql as $$ select p, p::text $$;
create function null_row() returns one_field language sql as $$ select null::one_field $$;
create function many_text(p text) returns setof text language sql as $$ select p union all select p $$;
create function table_pair(p int) returns table (a int, b text) language sql as $$ select p, p::text where p > 0 $$;
create function null_or_raise(p int) returns int language plpgsql as $$ begin if p > 1 then raise exception 'no'; end if; return null; end $$;
create function vol() returns bigint language sql volatile as $$ select (random() * 3)::bigint $$;
create function either(p int) returns int language sql as $$ select p $$;
create function either(p text) returns setof text language sql as $$ select p where false $$;
