select u.name from books p left join authors u on u.id = p.author_id where u.id = :id
