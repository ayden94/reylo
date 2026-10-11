select * from json_to_record('{"a": 1}') as r (a int, b text)
