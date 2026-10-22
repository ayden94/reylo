insert into accounts (id, email, nickname, note) select :id, :email, :nickname, :note returning id
