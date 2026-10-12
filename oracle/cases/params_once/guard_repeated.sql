select id from users where (:min::int is null or age > :min)
