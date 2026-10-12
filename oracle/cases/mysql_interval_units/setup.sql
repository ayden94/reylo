-- MySQL's compound interval units (MINUTE_SECOND, DAY_HOUR, YEAR_MONTH, …), which the engine's
-- parser does not know: each is read as its smallest part, so the statement keeps its row count
-- and its types (found by sqlc's corpus on 2026-10-11, where a statement with one was `many`).
create table events (id int primary key, created datetime not null, day date not null, at timestamp(6) null);
