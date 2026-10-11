select id, match (name) against (:q) as score from users where match (name) against (:q)
