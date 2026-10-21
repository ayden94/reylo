-- Nullability the WHERE, an inner join, a CASE arm, an aggregate's FILTER or a window proves.
create table teams (id int primary key, name text not null);
create table users (id int primary key, name text not null, nickname text, age int, team_id int references teams (id));
create table posts (id int primary key, author_id int references users (id), title text not null);
