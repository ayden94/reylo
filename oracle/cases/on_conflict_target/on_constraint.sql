insert into bar (id, name) values (:id, :name) on conflict on constraint bar_pkey do nothing
