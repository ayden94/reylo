insert into users as old (id, name) values (:id, :name) returning old.name as from_table, new.name as new_name
