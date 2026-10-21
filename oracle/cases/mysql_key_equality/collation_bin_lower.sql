select bin from texts ignore index (bin) where bin = lower(:v)
