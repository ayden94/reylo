select (select coalesce(sum(length(title)), 0) from posts where false) as n from users
