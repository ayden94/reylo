update users set name = :name where id = :id returning with (old as o) old.name
