select (select count(users) from posts where false) as n from users
