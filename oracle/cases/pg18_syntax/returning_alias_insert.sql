insert into users (id, name) values (:id, :name) returning with (old as o, new as n) o.name as old_name, n.name as new_name
