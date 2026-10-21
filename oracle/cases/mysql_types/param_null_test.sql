with input as (select :v as v) select id from everything, input where (input.v is null or everything.v = input.v)
