insert into app.tickets (id, status, code) values (:id, :status, :code) returning id, status, code
