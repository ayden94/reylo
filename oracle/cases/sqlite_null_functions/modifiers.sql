select date('now', fmt) as d, strftime(fmt, 'now') as f, unixepoch('now', fmt) as u from o
