select lag(:a) over () as a, first_value(coalesce(:b, whole)) over () as b, last_value(ifnull(price, :c)) over () as c from everything where whole between 0 and 2
