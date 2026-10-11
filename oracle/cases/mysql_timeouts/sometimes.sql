select id from bar where id = :id and sleep(case when :id = 2 then 60 else 0 end) = 0
