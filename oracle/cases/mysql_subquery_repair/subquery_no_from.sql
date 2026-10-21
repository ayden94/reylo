select u.id, (select 1) as one, (select u.name) as same, (select count(*)) as c from users u
