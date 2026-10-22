select a.id, jsonb_agg(b) as later from users a left join users b on b.id > a.id group by a.id
