select (select count(case when name is null then 1 end) from posts where false) as n from users
