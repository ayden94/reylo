-- MySQL reads a backslash in a string as an escape, so `:not` is inside the string, and `#` starts a comment.
select id, 'it\'s :not' as said, 'C:\\' as path
from users
where id = :id # and name = :nobody
