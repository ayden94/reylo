select count(*) as c, (select count(*) from posts) as n from users
