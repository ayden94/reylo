select author_id, count(*) as n from posts where author_id = :a group by author_id
