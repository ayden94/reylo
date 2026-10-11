select (select count(k) from json_to_record('{}') as r (k int, v text) where false) as n from users
