select e.id, h from events e, generate_series(e.at, e.at + interval '2 hours', interval '1 hour') h
