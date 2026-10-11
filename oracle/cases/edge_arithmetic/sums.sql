select sum(n)::int4 as total, sum(n + n) as doubled, sum(took) as took from spans where n >= :least
