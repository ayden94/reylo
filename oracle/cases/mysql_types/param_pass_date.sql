select coalesce(:d, day) as a, if(flag, :e, at) as b, greatest(day, :f) as c, ifnull(ts, :g) as d, least(:h, t) as e, case when flag then day else :i end as f from everything
