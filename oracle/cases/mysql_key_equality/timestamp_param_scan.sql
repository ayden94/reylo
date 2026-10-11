select ts from events ignore index (ts) where ts = :ts
