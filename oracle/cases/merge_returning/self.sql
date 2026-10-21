merge into users u using users s on u.id = s.id + 1 when matched then update set nick = s.name when not matched by source then update set nick = null returning s.name as prev, u.name as cur
