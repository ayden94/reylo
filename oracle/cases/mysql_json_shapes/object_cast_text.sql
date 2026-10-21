select json_object('a', cast('[1, 2]' as json), 'b', cast(id as json), 'c', cast(born as json), 'd', cast(id = 1 as json)) as o from users
