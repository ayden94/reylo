-- Arithmetic an edge value makes fail on every run: two infinite dates cannot be subtracted, an
-- infinite timestamp less itself is no interval, and NaN is no integer. The state seeded with
-- those values checks none of these statements; the plain state, seeded without infinities and
-- NaN, lets them run.
create table spans (id int primary key, opened date not null, closed date, at timestamptz, took interval, amount numeric not null, n int not null);
