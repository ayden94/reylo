select (select count(id) from posts join comments using (id) where false) as n from users
