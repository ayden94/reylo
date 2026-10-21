select u.id, exists (select 1 from posts p where p.user_id = u.id) as has_posts from users u
