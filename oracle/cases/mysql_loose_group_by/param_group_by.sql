select id + :n as bucket, count(*) as c from users where id between 0 and 2 group by id + :n
