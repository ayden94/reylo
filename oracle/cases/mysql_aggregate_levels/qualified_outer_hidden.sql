select (select count(posts.id) from posts p where false) as n from users posts
