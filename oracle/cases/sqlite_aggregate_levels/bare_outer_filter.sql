select (select count(*) filter (where name <> '') from posts where 0) as n from users
