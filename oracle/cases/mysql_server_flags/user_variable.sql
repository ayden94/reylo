select @counter := coalesce(@counter, 0) + 1 as seen, @other as nothing from t limit 3
