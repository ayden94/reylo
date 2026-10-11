delete from users using other where other.id = users.id and users.id = :id returning other.name as other_name, old.name as old_name, new.name as new_name, users.name as name
