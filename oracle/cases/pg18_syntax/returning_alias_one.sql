update users set name = :name where id = :id returning with (old as o) o.name as old_name, new.name as new_name
