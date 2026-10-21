with my_sequence as (select id as last_value from items) select last_value from my_sequence
