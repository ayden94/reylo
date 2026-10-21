select x.name from (select b.name from people a left join people b on b.id = a.boss) x, people u where u.id = :id
