select whole + :n as bucket, count(*) as c from everything where whole between 0 and 2 group by whole + :n
