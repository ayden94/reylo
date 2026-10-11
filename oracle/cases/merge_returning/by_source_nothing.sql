merge into users u using other s on u.id = s.id when matched then update set name = s.name when not matched by source then do nothing returning s.name as source_name, label, s.name || '!' as shout
