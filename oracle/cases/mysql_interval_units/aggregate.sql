select count(*) as n from events where created < now() - interval '1 1' day_hour
