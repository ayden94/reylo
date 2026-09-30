update pgf_users set email = :email, name = :name
where (cast(:role as pgf_role) is null or role = :role)
returning id, email
