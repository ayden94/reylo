select json_object('next', id + 1, 'upper', upper(name), 'len', length(name), 'half', id / 2, 'null', null, 'one', 1, 'text', 'x', 'now', now(), 'sum', score + 1) as o from users
