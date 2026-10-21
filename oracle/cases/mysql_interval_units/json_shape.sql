select json_object('later', date_add(created, interval '1:30' hour_minute), 'day', day + interval '1-1' year_month) as doc from events where id = :id
