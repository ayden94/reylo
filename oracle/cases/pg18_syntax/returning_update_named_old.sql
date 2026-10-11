update old set name = :name where id = :id returning old.name as from_table, new.name as new_name
