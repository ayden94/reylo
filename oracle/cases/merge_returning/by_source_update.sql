merge into users u using other s on u.id = s.id when not matched by source then update set nick = 'gone' returning s.name as source_name, label, u.nick as nick, u.name || '!' as shout
