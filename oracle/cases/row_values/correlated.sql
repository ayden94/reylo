select p.id, array(select c from docs c where c.id = p.id) as rows, (select c from docs c where c.id = p.id) as one from docs p
