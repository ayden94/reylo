insert into bookings (room, during, who) values (:room, :during, :who) on conflict do nothing returning who
