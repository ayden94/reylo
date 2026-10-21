select s.m from (select mode() within group (order by name) as m from authors) s
