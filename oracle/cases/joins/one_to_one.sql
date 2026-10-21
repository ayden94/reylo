select u.email, pr.bio from users u left join profiles pr on pr.user_id = u.id where u.id = :id
