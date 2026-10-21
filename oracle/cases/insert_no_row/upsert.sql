insert into kv values (:k, :v) on conflict (k) do update set v = excluded.v returning k
