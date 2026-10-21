with ins as (insert into bar (id, name) values (:id, :name) returning id, name) select ins.id, u.id as user_id from ins left join users u on u.email = ins.name
