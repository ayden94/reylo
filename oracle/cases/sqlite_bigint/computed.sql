select user_id - 1 as minus, user_id * n as times, -ref as neg, abs(big) as a, cast(wide as integer) as ci, coalesce(ref, 1.5) as cr, user_id + 0.5 as half
from accounts
