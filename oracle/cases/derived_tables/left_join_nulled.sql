select b.id, s.n from bar b left join (select count(*) as n from bar) s on false where b.id = :id
