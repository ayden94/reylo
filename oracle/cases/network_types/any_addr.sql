select id, host(addr) as host, masklen(net) as bits, family(addr) as family from hosts where addr = any(:addrs)
