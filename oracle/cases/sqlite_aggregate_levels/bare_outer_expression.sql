select (select coalesce(max(length(name)), 0) from posts where 0) as n from users
