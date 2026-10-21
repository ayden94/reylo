select tenant, at from stamped ignore index (primary) where tenant = :tenant and at = :at
