select json_object('i', 1, 'id', id, 'big', github_id, 'r', score, 'inf', score * 1e308, 't', name, 'e', email, 'n', n, 'nul', null, 'cmp', id > 1, 'tru', true, 'flag', active, 'any', extra, 'doc', doc) as o
from users
