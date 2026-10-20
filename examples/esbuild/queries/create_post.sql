insert into posts (user_id, title, body)
values (:user_id, :title, :body)
returning id, user_id, title, body
