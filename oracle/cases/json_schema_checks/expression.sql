select doc -> 'name' as name, doc || '{}'::jsonb as merged from docs
