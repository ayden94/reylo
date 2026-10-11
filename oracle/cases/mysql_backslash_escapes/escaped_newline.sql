select 'a\nb' as s, char_length('a\nb') as n, '\t\0\Z\%\_' as t, id from users where id = :id
