select (select total(length(title)) from posts where 0) as n from users
