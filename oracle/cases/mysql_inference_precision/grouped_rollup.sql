select kind, json_arrayagg(id) as ids from events where id > 100 group by kind with rollup
