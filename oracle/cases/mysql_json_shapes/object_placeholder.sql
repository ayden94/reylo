select json_object('n', :n + 1, 's', :s, 'id', id) as o from users where id = :id
