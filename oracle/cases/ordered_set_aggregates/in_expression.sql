select coalesce(mode() within group (order by bio), 'none') as top from authors
