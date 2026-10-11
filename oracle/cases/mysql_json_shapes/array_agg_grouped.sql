select user_id, json_arrayagg(title) as titles from posts group by user_id
