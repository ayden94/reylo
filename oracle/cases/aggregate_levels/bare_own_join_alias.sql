select (select count(body) from (posts join comments using (id)) as j where false) as n from users
