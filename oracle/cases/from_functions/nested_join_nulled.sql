select u.id, a.a, b.b from users u left join (now() a cross join lower(:s::text) b) on false where u.id = :id
