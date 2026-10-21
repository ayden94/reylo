select u.email, x.title from users u cross join lateral (select title from posts where author_id = u.id order by id desc limit 1) x where u.id = :id
