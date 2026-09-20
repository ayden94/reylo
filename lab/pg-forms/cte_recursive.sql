with recursive counter(n) as (
  select 1
  union all
  select n + 1 from counter where n < 5
)
select n from counter
