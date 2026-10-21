select date_add(day, interval '1-2' year_month) as later from events where id = :id
