select users.id, s.tag from users, (select :tag as tag) as s where s.tag is null or users.bio = s.tag
