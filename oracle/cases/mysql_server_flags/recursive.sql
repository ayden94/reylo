with recursive r (n, s) as (select 1, cast('a' as char(10)) union all select n + 1, concat(s, 'a') from r where n < 3) select n, s from r
