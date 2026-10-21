select b.id, s.n from bar b cross join lateral (select count(*) as n from bar x where x.id > b.id) s where b.id = :id
