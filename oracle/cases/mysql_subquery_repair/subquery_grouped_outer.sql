select u.name, count(*) as c, (select count(*) from posts p where p.title = u.name) as n from users u group by u.name
