select id from accounts where user_id = :user_id and ref in (:a, :b) and big between :lo and :hi
