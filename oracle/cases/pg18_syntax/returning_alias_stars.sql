delete from users where id = :id returning with (new as n, old as o) n.*, o.*
