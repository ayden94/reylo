-- A LIMIT and an OFFSET only remove rows: a key's bound stands under any of them, and exactly one
-- row stays one only under a constant LIMIT of a row or more with no OFFSET.
create table bar (id bigint primary key, name text);
