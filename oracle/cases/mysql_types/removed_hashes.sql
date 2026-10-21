-- MySQL 9.7 and 26.7 have no md5, and refuse this statement.
select md5(v) as m from everything
