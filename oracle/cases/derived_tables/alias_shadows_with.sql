with s as (select count(*) as n from bar) select s.n from (select max(score)::bigint as n from bar where false) s
