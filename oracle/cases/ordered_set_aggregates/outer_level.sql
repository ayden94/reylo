select (select mode() within group (order by a.name) from authors b where false) as m from authors a
