select u.id, (select bit_or(p.id) from posts p where p.user_id = u.id) as bits, (select bit_and(p.id) from posts p where p.user_id = u.id) as all_bits from users u
