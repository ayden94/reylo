-- A call with WITHIN GROUP is an aggregate, the server's ordered-set and hypothetical-set ones
-- and a user's alike: one row with no GROUP BY, over no rows too.
create table authors (id bigint primary key, name text not null, bio text);
create function first_trans(state text[], v text) returns text[] language sql as $$ select array_append(state, v) $$;
create function first_final(state text[]) returns text language sql as $$ select state[1] $$;
create aggregate first_in_order(order by text) (sfunc = first_trans, stype = text[], finalfunc = first_final);
