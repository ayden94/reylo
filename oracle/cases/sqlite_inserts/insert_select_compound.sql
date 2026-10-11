insert into foo (name, k) select name, :a from bar union all select :b, n from bar returning name, k
