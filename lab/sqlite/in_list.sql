select id, name from users where id in (select value from json_each(:ids))
