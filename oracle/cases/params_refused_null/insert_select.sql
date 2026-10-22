insert into accounts (id, email, nickname, note, tag) select :id, :email, :nickname, :note, :tag returning id
