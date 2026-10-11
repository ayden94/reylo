select (select coalesce(max(p.title), json_object('a', 1)) from posts p) as o
