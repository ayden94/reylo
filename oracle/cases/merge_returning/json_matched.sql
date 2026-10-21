merge into users u using other s on u.id = s.id when matched then update set name = s.name returning json_build_object('source', s.name, 'target', u.name, 'row', to_jsonb(s)) as j
