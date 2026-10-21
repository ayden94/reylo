select big + ubig as mixed, ubig + 1 as past, utiny - small as under from everything where big between 0 and 2 and ubig between 0 and 2 and small <= utiny
