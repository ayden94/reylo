select bin from texts ignore index (bin) where bin = :v collate utf8mb4_general_ci
