-- MySQL 9.7 and 26.7 have no md5, and there a stored function of the name answers the call.
select id from users where email = md5(:e)
