merge into users u using other s on u.id = s.id when matched then delete returning old.name as old_name, new.name as new_name, u.name as cur, s.name as source_name
