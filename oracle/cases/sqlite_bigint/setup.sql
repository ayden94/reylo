-- A column declared BIGINT, INT8 or UNSIGNED BIG INT is int8 (wiki/driver-decoding.md, 2026-10-20):
-- the declared width decides, and the run time reads it as a BigInt, exact past 2^53 on every
-- driver. The edge seeds put SQLite's largest integer, 2^63 - 1, in each, and the oracle holds a
-- column typed int8 to SQLite's INTEGER storage class: an expression typed int8 passes an integer
-- through, and one that may give a real (`coalesce(ref, 1.5)`) is not int8. INTEGER, INT and the
-- rowid stay SQLite's INTEGER, a number, and so does what SQLite computes from a 64-bit column.
create table accounts (id integer primary key, user_id bigint not null, ref INT8, big unsigned big int, wide bigint(20), n integer not null, r real);
create table keyed (id bigint primary key, name text);
create view account_users as select user_id, n from accounts;
