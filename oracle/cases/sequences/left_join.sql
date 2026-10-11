select s.last_value, i.name from my_sequence s left join items i on i.id = :id
