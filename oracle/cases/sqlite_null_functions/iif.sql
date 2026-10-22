select iif(n > 0, placed_at) as a, iif(n > 0, placed_at, n > 1, fmt) as b, iif(n > 0, placed_at, n > 1, fmt, 'z') as c from o
