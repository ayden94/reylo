insert into kv values (:k, :v) on conflict (k) do update set v = excluded.v where kv.v > 100 returning k
