insert into parted (id, region) values (:id, :region) on conflict (id, region) do nothing
