select id from users where id = :id and exists (select 1 from posts where user_id = users.id)
