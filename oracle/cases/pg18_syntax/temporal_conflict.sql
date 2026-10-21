insert into bookings (room, during, who) values (:room, :during, :who) on conflict (room, during) do nothing returning who
