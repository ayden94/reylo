select (select count(ctid) from posts where false) as n from users
