select at from events ignore index (at) where at = concat(:a, :b)
