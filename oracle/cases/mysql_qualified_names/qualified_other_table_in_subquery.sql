select u.id, (select count(email) from reylo_oracle_app.users where false) as n from users u
