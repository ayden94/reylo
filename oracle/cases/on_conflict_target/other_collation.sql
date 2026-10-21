insert into bar (id, tag) values (:id, :tag) on conflict (tag) do nothing
