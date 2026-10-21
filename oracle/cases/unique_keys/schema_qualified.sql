select a.name, (select count(*) from users) as n from archive.users a where a.id = :id
