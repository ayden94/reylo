with gone as (delete from posts where id = :id returning *) select u, jsonb_agg(g) as gone from users u left join gone g on g.user_id = u.id group by u
