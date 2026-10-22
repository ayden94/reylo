select p.published_at, count(*) as n from posts p where p.published_at is not null group by p.published_at with rollup
