select (select coalesce(mode() within group (order by a.name), 'x') from authors b where false) as m from authors a
