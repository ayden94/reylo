select (select bit_or(length(name)) from posts where false) as n from users
