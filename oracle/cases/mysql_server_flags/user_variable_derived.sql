select d.seen, d.label from (select @counter := coalesce(@counter, 0) + 1 as seen, 'a@b' as label from t limit 3) d
