-- Unique indexes over an expression and with a predicate: each bounds only the reads it covers.
create table accounts (id bigint primary key, email text not null, deleted_at timestamptz, handle text);
create unique index accounts_email_live on accounts (email) where deleted_at is null;
create unique index accounts_lower_handle on accounts (lower(handle));
create table tags (id bigint primary key, name varchar(40), status text);
create unique index tags_name_active on tags (name) where status = 'active';
