with gone as (delete from users where id = :id returning with (old as o, new as n) o.name as was, n.name as now) select was, now from gone
