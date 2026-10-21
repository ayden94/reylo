select (select coalesce(max(length(name)), 0) from posts where false) as n from users
