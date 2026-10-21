select (select count(P.id) from posts p where false) as n from users P
