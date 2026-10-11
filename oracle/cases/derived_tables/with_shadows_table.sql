with tags as (select max(label) as label from tags where false) select tags.label from tags
