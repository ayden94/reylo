replace into users (id, name, bio) values (:id, :name, cast(:bio as char))
