select date_add(created, interval :span hour_minute) as later from events where id = :id
