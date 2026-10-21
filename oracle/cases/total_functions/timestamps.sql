select to_char(ts, 'YYYY') as y, extract(month from ts) as m, date_part('day', ts) as d from rt
