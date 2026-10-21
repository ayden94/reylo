-- Tables no rollback restores, and a statement that leaves an AUTO_INCREMENT counter at its
-- type's largest value: after a run that changed one the oracle builds the database again, so
-- the runs after it see the seeded rows.
create table hits (id int primary key auto_increment, page varchar(20) not null, n int not null default 0) engine = MyISAM;
create table cache (k varchar(20) primary key, v varchar(20)) engine = MEMORY;
create table users (id int primary key auto_increment, name varchar(20) not null);
