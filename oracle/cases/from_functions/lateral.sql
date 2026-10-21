select u.id, l from users u cross join lateral lower(u.email) l where u.id = :id
