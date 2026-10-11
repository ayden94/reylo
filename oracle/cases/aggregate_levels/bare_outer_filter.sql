select (select count(*) filter (where name <> '') from posts where false) as n from users
