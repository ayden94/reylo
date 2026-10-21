select :at + interval 1 day as next, date_add(:start, interval 1 hour) as later, datediff(:a, :b) as days, year(:c) as y
