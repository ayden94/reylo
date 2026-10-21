-- Every column type as the protocol reports it, the type the server gives a placeholder in each
-- context, and what a write returns: nothing, MySQL having no RETURNING. A column an expression
-- computes from a placeholder has the type the server gave it at the prepare, and an execution
-- answers with the type of the value that was bound.
create table everything (
  id int primary key auto_increment,
  tiny tinyint, small smallint, medium mediumint, whole int, big bigint,
  utiny tinyint unsigned, usmall smallint unsigned, umedium mediumint unsigned, uwhole int unsigned, ubig bigint unsigned,
  flag boolean not null default true,
  price decimal(10,2), exact decimal(40,20), f float, d double,
  c char(5), v varchar(20), tt tinytext, tx text, mt mediumtext, lt longtext,
  b binary(3), vb varbinary(8), tb tinyblob, bl blob, mb mediumblob, lb longblob,
  kind enum('a','b','c'), flags set('x','y','z'),
  day date, at datetime, at6 datetime(6), ts timestamp null, t time, y year,
  doc json, bits bit(8), one_bit bit(1), pt point
);
create table notes (id int primary key auto_increment, body varchar(20) not null, author varchar(20), score int not null default 0, added datetime not null default current_timestamp, extra json, g int generated always as (score + 1) stored);
