select r.id, r.title from recipe_index join recipes r on r.id = recipe_index.rowid where recipe_index match :q
