select (select coalesce(sum(length(name)), 0) from posts where false) as n from users
