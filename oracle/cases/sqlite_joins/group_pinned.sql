select user_id, count(*) as n from posts where user_id = :u group by user_id
