insert into deferred (id, name) values (:id, :name) on conflict (name) do nothing
