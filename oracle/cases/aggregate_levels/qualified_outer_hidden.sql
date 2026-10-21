select (select count(posts.title) from (posts join comments using (id)) as j where false) as n from users posts
