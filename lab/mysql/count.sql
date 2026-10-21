select count(*) as n, max(at) as latest
from users
where name like :pattern
