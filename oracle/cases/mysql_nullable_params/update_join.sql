update users u join posts p on p.user_id = u.id set u.bio = :bio, p.title = :title, p.tag = :tag where p.id = :id
