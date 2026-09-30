import mysql from "mysql2/promise";
const c = await mysql.createConnection(process.env.DATABASE_URL ?? "mysql://root:porg@127.0.0.1:33306/porg");
await c.query("drop table if exists posts");
await c.query("drop table if exists users");
await c.query(`create table users (id int primary key auto_increment, name varchar(50) not null, email varchar(100), big bigint not null default 9007199254740993, n decimal(10,2) not null default 1.50, at datetime not null default current_timestamp, j json, flag tinyint(1) not null default 1, raw varbinary(8))`);
await c.query(`insert into users (name, email, j) values ('ada', 'ada@example.com', '{"a":1}'), ('bob', null, null)`);
await c.query(`create table posts (id int primary key auto_increment, user_id int not null references users (id), title varchar(100) not null, tag varchar(20))`);
await c.query(`insert into posts (user_id, title, tag) values (1, 'hello', 'intro'), (1, 'again', null)`);
await c.end();
