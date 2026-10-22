insert into accounts (id, email, nickname) values (:id, :email, :nickname) on duplicate key update note = :note
