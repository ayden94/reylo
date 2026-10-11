insert into users (id, name) values (:id, :name) returning old.name::text as old_name, (old.name)::varchar as old_v, new.name::text as new_name
