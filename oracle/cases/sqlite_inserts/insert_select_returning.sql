insert into foo (name, meta, k) select name, :meta, n + :d from bar where n > :n returning name, meta, k
