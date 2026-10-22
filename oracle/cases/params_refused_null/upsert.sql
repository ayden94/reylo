insert into accounts (id, email, nickname, tag) values (:id, :email::text, :nickname::text, :tag::text) on conflict (id) do update set email = excluded.email, note = :note returning id
