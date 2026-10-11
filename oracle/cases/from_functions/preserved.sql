select t.t, u.email from now() t left join users u on u.id = :id
