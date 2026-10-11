select name from prefixes ignore index (name_prefix) where name = :name
