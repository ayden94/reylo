select count(*) as n, sum(count(*)) over () as total from users
