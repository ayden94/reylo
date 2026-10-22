select lag(user_id) over (order by id) as prev, first_value(wide) over (order by id) as first, min(user_id, ref) as smaller from accounts
