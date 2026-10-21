select u.id, array_agg(u.nickname) filter (where u.nickname is not null) as nicks from users u group by u.id
