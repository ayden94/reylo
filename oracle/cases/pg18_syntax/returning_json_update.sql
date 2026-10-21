update users set name = :name where id = :id returning json_build_object('old', old.name, 'new', new.name, 'row', to_jsonb(old)) as j
