select p.title, count(*) as n from users u join posts p on p.user_id = u.id where u.id = :id group by p.title
