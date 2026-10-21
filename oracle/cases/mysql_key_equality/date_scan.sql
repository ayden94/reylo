select day from events ignore index (day) where day = :day
