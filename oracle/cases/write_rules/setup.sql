-- A rewrite rule replaces a write with its own actions, and a view hands it to its base table,
-- whose triggers and rules run (the soundness review of 2026-10-04): RETURNING can then give any
-- number of rows.
create table t (id int8 primary key, v text);
create table t2 (id int8, v text);
create rule r as on insert to t do instead insert into t2 select new.id, new.v from generate_series(1, 3) returning t2.*;
create table u (id int8 primary key, v text);
create table u2 (id int8, v text);
create rule r0 as on insert to u do instead insert into u2 select new.id, new.v where false returning u2.*;
create table vt (id int8 primary key, v text);
create function skip_row() returns trigger language plpgsql as $$ begin return null; end $$;
create trigger vt_skip before insert on vt for each row execute function skip_row();
create view vv as select id, v from vt;
create table ut (id int8 primary key, v text);
create table ut_log (id int8, v text);
insert into ut values (1, 'a');
create rule ru as on update to ut do instead insert into ut_log select new.id, new.v from generate_series(1, 3) returning ut_log.*;
create table plain (id int8 primary key, v text);
