-- Under NO_BACKSLASH_ESCAPES a backslash is a character like any other, so `'x\'` is a whole
-- string and what follows it is SQL.
create table users (id int primary key, email varchar(40) not null unique, name varchar(40) not null);
