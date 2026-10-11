select t from events ignore index (t) where t = :t
