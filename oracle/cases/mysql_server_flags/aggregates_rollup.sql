select n, small, count(*) as a, sum(small) as b, grouping(n) as c, max(name) as d from t where n between -1 and 2 group by n, small with rollup
