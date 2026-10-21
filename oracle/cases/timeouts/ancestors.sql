with recursive up as (select id, parent_id, name from nodes where id = :id union all select n.id, n.parent_id, n.name from nodes n join up on n.id = up.parent_id) select id, name from up
