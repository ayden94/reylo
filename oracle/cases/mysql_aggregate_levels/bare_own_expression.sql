select (select coalesce(max(length(title)), 0) from posts where false) as n from users
