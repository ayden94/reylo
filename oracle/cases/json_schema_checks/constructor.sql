select json_build_object('id', d.id, 'doc', d.doc, 'flags', p.f) as o from docs d left join prefs p on p.id = d.id
