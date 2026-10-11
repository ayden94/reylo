select a.n, t.t, s.last_value from (select count(*) as n from bar) a, now() t, seq s
