update users set name = :name where id = :id returning with (old as o, new as n) to_jsonb(o) as before, to_jsonb(n) as after, o as old_row, (n).name as n_name
