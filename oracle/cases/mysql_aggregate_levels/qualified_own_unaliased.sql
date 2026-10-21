select (select count(posts.title) from posts where false) as n from users
