with r as (select c from docs c) select jsonb_agg(r.c) as rows from r
