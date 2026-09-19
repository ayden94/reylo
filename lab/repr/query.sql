select id, created_at, big, created_at::date as day, created_at::timestamp as local, array[created_at] as stamps, count(*) over () as n
from lando_users where id = :id
