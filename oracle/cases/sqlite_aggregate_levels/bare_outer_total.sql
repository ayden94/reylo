select (select total(length(name)) from posts where 0) as n from users
