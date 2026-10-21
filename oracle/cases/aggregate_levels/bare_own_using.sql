select (select count(body) from posts join comments using (id) where false) as n from users
