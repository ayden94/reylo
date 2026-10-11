insert into parted (id, region) values (:id, :region) on conflict (id) do nothing
