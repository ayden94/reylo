select u.id, t.t from users u left join now() t on false where u.id = :id
