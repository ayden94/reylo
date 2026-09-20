select id, name from pgf_users where name = any(:names) order by id
