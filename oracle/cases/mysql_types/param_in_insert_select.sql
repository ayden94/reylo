insert into notes (body, score) select concat(coalesce(v, 'x'), :suffix), coalesce(whole, :fallback) + :bump from everything where whole between 0 and 2
