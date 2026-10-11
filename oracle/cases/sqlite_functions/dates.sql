select date('now') as d, strftime('%Y', current_timestamp) as y, unixepoch() as u, julianday('now') as jd, datetime(0, 'unixepoch') as zero
