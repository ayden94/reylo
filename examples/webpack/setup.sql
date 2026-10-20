drop table if exists posts;
drop table if exists users;
create table users (
  id int primary key auto_increment,
  name varchar(100) not null,
  email varchar(255),
  role enum('admin', 'member') not null default 'member',
  created_at datetime not null default current_timestamp
);
create table posts (
  id int primary key auto_increment,
  user_id int not null,
  title varchar(200) not null,
  body text,
  foreign key (user_id) references users (id)
);
insert into users (name, email, role) values ('ada', 'ada@example.com', 'admin'), ('bob', null, 'member');
insert into posts (user_id, title, body) values (1, 'Hello', 'first post'), (1, 'Again', null);
