insert into deferred (id, name) values (:id, :name) on conflict (id) do nothing
