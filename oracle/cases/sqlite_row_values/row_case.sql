select id, case (created_at, id) when (:created_at, :id) then 'first' else 'other' end as tag from users
