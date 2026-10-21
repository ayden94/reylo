insert into posts (user_id, title, body) values (:user_id, :title, :body)
returning json_object('id', id, 'title', title, 'body', body, 'tags', json_array(title, body)) as post
