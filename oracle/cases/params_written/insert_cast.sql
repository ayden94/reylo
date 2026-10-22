insert into users (id, name, email, score, n) values (:id::int, :name::text, :email::text, :score::numeric, :n::int) on conflict (id) do update set name = excluded.name returning id
