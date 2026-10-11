select name from users order by (select count(*) from posts)
