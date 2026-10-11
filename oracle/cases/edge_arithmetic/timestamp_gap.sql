select a.id, a.at - b.at as gap from spans a join spans b on b.id = a.id
