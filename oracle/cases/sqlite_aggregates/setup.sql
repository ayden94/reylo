-- sum() over integers raises "integer overflow" once the largest integer stands beside any
-- positive value, so a state seeded with it checks no statement that sums the column: the plain
-- state, seeded without extreme magnitudes, lets the aggregates run.
create table orders (id integer primary key, customer integer not null, cents integer not null, weight real);
