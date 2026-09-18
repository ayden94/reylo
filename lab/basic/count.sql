select count(*) as n, max(created_at) as latest from lando_users where role = :role
