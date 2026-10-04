select t.id, array_agg(m.email) as emails from teams t left join members m on m.team_id = t.id group by t.id
