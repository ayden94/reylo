select +:a as a, (:b) as b, :c as c, coalesce(:d, 'x') as d, coalesce(:e, 1) as e, coalesce(:f, 1.5) as f, ifnull(:g, now()) as g
