select id, name from pgf_users
where email like :q or name like :q
