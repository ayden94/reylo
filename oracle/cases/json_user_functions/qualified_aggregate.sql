select coalesce(app.json_agg(name), '[]') as j from users
