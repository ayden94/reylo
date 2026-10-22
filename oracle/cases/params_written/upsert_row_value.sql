insert into users (id, name) values (:id::int, 'x') on conflict (id) do update set (email, score) = (:email::text, :score::numeric) returning email, score
