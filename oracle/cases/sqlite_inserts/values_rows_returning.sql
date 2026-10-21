insert into users (name, nick) values (:a, :n), (:b, :m), (:c, upper(:u)) returning id, name, nick
