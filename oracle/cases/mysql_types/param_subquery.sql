select (select :n + whole from everything order by id limit 1) as sub, (select max(:m) from everything) as mx
