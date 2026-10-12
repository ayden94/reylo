select extract(day_minute from created) as dm, extract(year_month from day) as ym from events where id = :id
