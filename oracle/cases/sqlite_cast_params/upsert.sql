insert into users (name, email) values (CAST(:name AS TEXT), CAST(:email AS TEXT)) on conflict (email) do update set name = excluded.name returning id
