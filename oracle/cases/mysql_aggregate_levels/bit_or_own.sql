select (select bit_or(length(title)) from posts where false) as n from users
