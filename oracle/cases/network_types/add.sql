insert into hosts (id, addr, net, mac, mac8) values (:id, :addr, :net, :mac, :mac8) returning id, addr, net, mac, mac8
