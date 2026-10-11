-- A derived table is what the same query is as a WITH query: its row count, and its columns'
-- nullability from its own statement, by name, by the names its alias gives, or through a `*`;
-- an outer join nulls it.
create table bar (id bigint primary key, name text, score int not null);
create table tags (id bigint primary key, label text not null);
create sequence s;
