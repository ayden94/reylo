select cs from texts ignore index (cs) where cs = current_user()
