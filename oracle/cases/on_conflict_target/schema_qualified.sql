insert into public.bar (id, name) values (:id, :name) on conflict (name) do nothing
