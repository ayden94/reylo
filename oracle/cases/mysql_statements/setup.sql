-- What is a statement the engine types: one SELECT, INSERT, UPDATE, DELETE, REPLACE, VALUES or
-- TABLE. A statement of a migration, or two statements, is an error and is never run; what the
-- server refuses to prepare is an error with the server's message; and a statement the engine's
-- own parser cannot read keeps its types from the server and loses the row count and the JSON
-- shapes. MySQL runs the text of a `/*! … */` comment as SQL.
create table users (id int primary key, name varchar(20) not null, email varchar(40) unique, doc json, fulltext key (name));
create table log (id int primary key auto_increment, line varchar(40) not null);
create procedure touch() begin insert into log (line) values ('touched'); end;
create function next_line() returns int deterministic reads sql data return (select count(*) from log);
create trigger users_log after insert on users for each row insert into log (line) values (concat('user ', new.id));
