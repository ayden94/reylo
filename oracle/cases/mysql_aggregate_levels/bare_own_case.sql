select (select count(case when title is null then 1 end) from posts where false) as n from users
