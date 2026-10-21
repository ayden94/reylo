-- An INSERT of one VALUES row returns no row when ON CONFLICT DO UPDATE's WHERE is false, or when
-- a BEFORE ROW trigger returns NULL.
create table kv (k int primary key, v int);
create table trg (id int primary key, v text);
create function skip_row() returns trigger language plpgsql as $$ begin return null; end $$;
create trigger trg_skip before insert on trg for each row when (new.v = 'skip') execute function skip_row();
create table plain (id int primary key, v text);
create table after_trg (id int primary key, v text);
create function noop() returns trigger language plpgsql as $$ begin return new; end $$;
create trigger after_noop after insert on after_trg for each row execute function noop();
create view kv_view as select k, v from kv;
create function view_insert() returns trigger language plpgsql as $$ begin return null; end $$;
create trigger kv_view_insert instead of insert on kv_view for each row execute function view_insert();
