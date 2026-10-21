select id, addr, mac from hosts where addr <<= :net::cidr
