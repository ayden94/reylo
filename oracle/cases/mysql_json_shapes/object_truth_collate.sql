select json_object('collate', name = 'a' collate utf8mb4_bin, 'whole', (name = 'a') collate utf8mb4_bin) as o from users
