-- Postgres arrays hold NULL elements whatever their column's constraints say.
create table teams (id int primary key, tags text[] not null default '{}', scores int[]);
create table members (id int primary key, team_id int references teams (id), email text not null, nickname text);
