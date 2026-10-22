-- Truth values (wiki/driver-decoding.md, 2026-10-20). MySQL has no boolean type: BOOLEAN is
-- TINYINT(1), which the prepare reports one digit wide, and a BIT(1) one bit wide. A table's such
-- column is typed bool and holds whatever was stored (the seeds put -128 and 127 in it), which the
-- run time refuses unless it is 0 or 1; a comparison, a test or a logical operator the statement
-- computes is typed bool too, and the oracle holds every value of one to 1, 0 or NULL. A view's
-- column, an expression over a flag column and a TINYINT(4) stay numbers.
create table flags (
  id int primary key,
  done boolean not null default false,
  maybe tinyint(1),
  unsigned_flag tinyint(1) unsigned,
  small tinyint,
  four tinyint(4),
  bit_flag bit(1),
  eight bit(8),
  a int,
  b int,
  label varchar(10)
);
create view open_flags as select id, done, a = b as same from flags where not done;
