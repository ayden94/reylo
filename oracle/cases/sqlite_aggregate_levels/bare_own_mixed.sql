select (select count(length(name) + length(title)) from posts where 0) as n from users
