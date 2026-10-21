select kind from kinds ignore index (kind) where kind = :kind
