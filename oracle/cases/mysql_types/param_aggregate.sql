select sum(whole + :n) as s, max(:m) as m, count(:c) as c, avg(:a) as a, min(coalesce(:d, day)) as d from everything where whole between 0 and 2
