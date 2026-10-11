select sum(:n) over () as total, lag(whole, :m) over (order by id) as previous from everything where whole between 0 and 2
