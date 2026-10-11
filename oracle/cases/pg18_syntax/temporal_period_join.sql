select s.guest, b.who from stays s join bookings b on b.room = s.room and b.during @> s.during where s.room = :room
