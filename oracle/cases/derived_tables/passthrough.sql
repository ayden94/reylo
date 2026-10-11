select s.name, s.score from (select name, score from bar where id = :id) s
