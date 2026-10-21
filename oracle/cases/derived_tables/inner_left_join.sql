select s.label from (select t.label from bar b left join tags t on false) s
