update users set o = :o where id = :id returning with (old as o) o as the_column, o.o as old_o, o.name as old_name
