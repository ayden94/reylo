select @`counter` := coalesce(@`counter`, 0) + 1.5 as seen, concat(@`other`, 'x') as nothing, n from t limit 3
