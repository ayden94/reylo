select (select count(title) from public.posts where false) as n from users
