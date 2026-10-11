-- Under the server's default sql_mode a backslash in a string escapes the character after it, in
-- single and double quotes alike: a placeholder after `\'` is inside the string, and one after
-- `'\\'` is not. `#` starts a line comment, `--` does only before a space or a control character,
-- and a block comment ends at its first `*/`.
create table users (id int primary key, name varchar(40) not null unique, email varchar(40));
