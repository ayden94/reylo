select to_jsonb(x) as j from users x, generate_series(1, 2) g
