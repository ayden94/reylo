select * from unnest(array[1, 2]) with ordinality as t (v, n)
