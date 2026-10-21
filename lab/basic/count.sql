select count(*) as n, max(created_at) as latest from porg_users where role = :role
