select (select count(name) from public.posts where false) as n from users
