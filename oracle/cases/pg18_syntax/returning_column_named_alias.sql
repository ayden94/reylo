insert into t2 (id, n) values (:id, :n) returning with (old as n) n as the_column, n.n as old_n, new.n as new_n
