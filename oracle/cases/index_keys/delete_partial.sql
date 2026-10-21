delete from accounts where email = :e and deleted_at is null returning id
