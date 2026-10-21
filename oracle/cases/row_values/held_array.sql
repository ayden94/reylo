select array(select h from holders h) as held_rows, array(select x.h from holders x) as held_values
