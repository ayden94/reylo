select distinct on (author_id) title from posts where author_id = :a order by author_id, id desc
