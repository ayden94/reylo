select jsonb_agg(x.c) as rows from (select c from docs c) x
