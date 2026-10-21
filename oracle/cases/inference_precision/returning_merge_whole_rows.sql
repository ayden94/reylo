merge into posts p using users u on p.id = u.id when matched then update set title = u.name when not matched by source then delete returning u as src, old as was, new as now
