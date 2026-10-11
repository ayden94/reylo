update users set name = :name where id = :id returning with (old as was, new as now) to_jsonb(was) as before, to_jsonb(now) as after, was as old_row, (now).name as now_name, row_to_json(now) as j
