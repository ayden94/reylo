with input as (select cast(:role as pgf_role) as role)
update pgf_users set email = :email, name = :name
from input
where (input.role is null or pgf_users.role = input.role)
returning id, email
