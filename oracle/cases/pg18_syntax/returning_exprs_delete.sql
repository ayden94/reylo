delete from users where id = :id returning old.name || '!' as old_shout, new.name || '!' as new_shout, coalesce(new.name, old.name) as either, name || '!' as shout
