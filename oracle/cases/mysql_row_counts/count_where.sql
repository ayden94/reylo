select count(*) as n, max(age) as oldest from users where name like :pattern
