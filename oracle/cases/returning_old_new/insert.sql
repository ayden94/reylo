insert into users (id, name) values (:id, :name) returning old.name as old_name, new.name as new_name
