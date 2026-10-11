select (select count(name) from posts join comments using (id) where false) as n from users
