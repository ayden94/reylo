insert into bar_view (id, name) values (:id, :name) on conflict (id) do nothing
