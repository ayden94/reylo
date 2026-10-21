select max(:a) as a, min(coalesce(:b, price)) as b, any_value(:c) as c, max(coalesce(:d, whole)) as d, min(ifnull(day, :e)) as e from everything
