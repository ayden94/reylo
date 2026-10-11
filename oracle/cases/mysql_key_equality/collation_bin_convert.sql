select bin from texts ignore index (bin) where bin = convert(:v using latin1)
