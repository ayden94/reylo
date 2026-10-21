select email, count(*) as n from users u join posts p on p.user_id = u.id where email = :email group by email
