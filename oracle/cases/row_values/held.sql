select jsonb_agg(h.h order by h.id) as held from holders h
