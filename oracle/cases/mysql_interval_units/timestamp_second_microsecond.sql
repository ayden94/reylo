select date_sub(at, interval '1.5' second_microsecond) as earlier from events where id = :id
