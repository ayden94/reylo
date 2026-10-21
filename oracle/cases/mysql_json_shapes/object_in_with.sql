with t as (select json_object('id', id, 'email', email) as o from users) select o from t
