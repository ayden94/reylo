insert into bar (id, email) values (:id, :email) on conflict (email) do nothing
