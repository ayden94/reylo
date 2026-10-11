select json_object('a', coalesce(:p, score), 'b', if(flag, :q, score), 'c', :r, 'd', coalesce(:d, born), 'e', greatest(id, :n), 'f', :m + 1, 'g', coalesce(:t, at)) as o from users
