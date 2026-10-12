select id, score from (select id, match (name) against (:q) as score from users) as s where score > 0
