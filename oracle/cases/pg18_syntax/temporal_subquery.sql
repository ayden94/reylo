select s.note, (select b.who from bookings b where b.room = s.room) as who from slots s where s.note = :note
