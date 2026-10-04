select u.name from books p left join authors u on u.id = p.author_id cross join (select 1 as id) as authors where authors.id = 1
