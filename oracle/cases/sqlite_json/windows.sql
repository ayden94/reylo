select id, json_group_array(id) over (order by id rows between 1 following and 1 following) as next, json_object('w', json_group_array(score) over ()) as w
from users
