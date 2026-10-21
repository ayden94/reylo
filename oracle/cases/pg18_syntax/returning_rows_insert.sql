insert into users (id, name) values (:id, :name) returning to_jsonb(old) as before, to_jsonb(new) as after, old as old_row, (old).name as old_name, (old).name || '!' as old_shout
