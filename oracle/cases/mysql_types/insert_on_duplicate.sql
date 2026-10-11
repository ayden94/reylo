insert into notes (id, body) values (:id, :body) on duplicate key update body = values(body), score = score + :bump
