select json_object('d', d.doc, 'n', (select json_arrayagg(e.payload) from events e)) as o from docs d
