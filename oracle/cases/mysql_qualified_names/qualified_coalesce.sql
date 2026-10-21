select reylo_oracle_app.coalesce(json_object('id', id), json_object()) as o from users
