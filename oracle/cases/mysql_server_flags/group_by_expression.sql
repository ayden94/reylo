select upper(name) as shout, count(*) as c from t group by upper(name)
