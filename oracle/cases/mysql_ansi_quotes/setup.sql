-- Under ANSI_QUOTES a double-quoted word is an identifier, where the server's default reads a
-- string, and under PIPES_AS_CONCAT `||` joins strings, where the default reads OR. The session
-- of this case runs under both (the `sql_mode` file).
create table users (id int primary key, email varchar(40) not null unique, name varchar(40) not null, at datetime not null unique);
