select n from t where n in (:a, :b) or name = :name or (maybe_n is null and :flag)
