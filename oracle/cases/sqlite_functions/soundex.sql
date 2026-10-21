select soundex(name) as s, soundex(email) as e from users where soundex(name) = soundex(:x)
