-- inet, cidr, macaddr and macaddr8 take no '1' and no 'a': each has a pool of its own, with an
-- IPv4 and an IPv6 address, a host inside a network and a /0 network. A type the oracle has no
-- value for (pin, a domain no seeded string satisfies, standing in for an extension's type such
-- as pgvector's vector) leaves its statement unchecked, and the oracle says so (unchecked.txt).
create table hosts (id int primary key, addr inet not null, net cidr, mac macaddr, mac8 macaddr8);
create domain pin as text check (value ~ '^[0-9]{4}$');
create table doors (id int primary key, pin pin not null);
