insert into users (email, name) values (:email, :name) on conflict (email) do nothing returning id
