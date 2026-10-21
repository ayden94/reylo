select i.id, s.last_value from items i left join my_sequence s on false where i.id = :id
