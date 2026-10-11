select (select count(name) from (posts join comments using (id)) as j where false) as n from users
