-- Unique indexes that do not make a lookup single-row: deferrable, on an inheritance parent, under
-- another collation, or compared through a lossy cast. A partitioned table's key does hold.
create table tags (id int primary key, k int not null, constraint tags_k_key unique (k) deferrable initially deferred);
create table parent (id int primary key, v text);
create table child () inherits (parent);
create collation ci (provider = icu, locale = 'und-u-ks-level2', deterministic = false);
create table emails (id int primary key, email text unique, folded text collate ci unique, ci_c text collate ci);
-- An index under another collation than the comparison's: unique under "C", compared under ci.
create unique index emails_ci_c on emails (ci_c collate "C");
create table nums (id int primary key, k numeric unique, b bigint unique);
create table ties (id int primary key, score int);
create schema archive;
create table archive.users (id int, name text);
create table users (id int primary key, name text);
create table orgs (region int not null, id int not null, name text, primary key (region, id)) partition by list (region);
create table orgs_1 partition of orgs for values in (1);
