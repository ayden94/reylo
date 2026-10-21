with one as (select * from users where id = :id) select o.email, p.title from one o join posts p on p.id = o.pinned_post
