select (select count(body) from posts join comments using (id) where 0) as n from users
