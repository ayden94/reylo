insert into accounts (id, email, nickname) values (:id, :email, :nickname) on conflict (id) do update set email = excluded.email, note = :note returning id
