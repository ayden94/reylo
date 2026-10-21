select id from bar where id = :id union all select id from bar where id = :other limit 5
