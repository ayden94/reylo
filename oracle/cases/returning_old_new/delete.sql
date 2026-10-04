delete from users where id = :id returning old.name as old_name, new.name as new_name
