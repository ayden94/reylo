select (select jsonb_build_object('a', name) from users limit 1) as j
