merge into users u using other s on u.id = s.id when matched then delete returning u.name as gone, s.name as source_name, u.name || '!' as shout
