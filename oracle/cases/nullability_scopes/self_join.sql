select b.name as boss_name from people a left join people b on b.id = a.boss where a.id = :id
