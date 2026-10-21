select id, status from app.tickets where status = any(:statuses)
