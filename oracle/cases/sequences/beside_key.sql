select i.name, s.last_value from items i, my_sequence s where i.id = :id
