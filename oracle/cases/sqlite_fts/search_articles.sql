select a.id, a.body from article_index join articles a on a.id = article_index.rowid where article_index match :q
