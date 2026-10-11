insert into notes (body) with t as (select :body as body) select body from t
