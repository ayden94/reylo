-- lower and upper of an empty or unbounded range, and to_char, extract and date_part of
-- 'infinity', are NULL for non-null arguments; the text, numeric and time overloads are not.
create table rt (id int primary key, r int4range not null, ts timestamp not null, n numeric not null, t text not null, tm time not null, sep text);
