merge into users u using other s on u.id = s.id when matched then update set name = s.name when not matched by source then delete returning s.*, u.*
