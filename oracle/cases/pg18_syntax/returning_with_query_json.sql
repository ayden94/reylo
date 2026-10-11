with gone as (delete from users where id > 0 returning new.name as now, old.name as was) select json_build_object('now', gone.now, 'was', gone.was) as j, upper(gone.now) as up, gone.was from gone
