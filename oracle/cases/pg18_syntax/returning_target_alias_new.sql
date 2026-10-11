delete from users as new where id = :id returning new.name as from_table, old.name as old_name
