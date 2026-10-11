-- MySQL 9.7 and 26.7 have no md5 and no sha1, and refuse this statement.
select md5(name) as p, sha1(name) as q from t where n between -1 and 2
