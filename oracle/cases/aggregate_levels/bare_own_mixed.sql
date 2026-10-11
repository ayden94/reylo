select (select count(length(name) + length(title)) from posts where false) as n from users
