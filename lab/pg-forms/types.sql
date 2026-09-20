select id, span, during, addr, price, bits, doc, search, pair, roles
from pgf_things
where id = :id
