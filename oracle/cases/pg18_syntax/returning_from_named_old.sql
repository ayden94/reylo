update users set nick = old.name from other as old where users.id = old.id returning old.name as from_other, new.name as new_name, users.nick as nick
