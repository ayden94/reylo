select json_arrayagg(json_object('id', u."id", 'email', u."email")) as l from "users" u where u."id" < 3
