insert into todos (title, done) values (:title, :done) returning id, done, done = 1 as finished
