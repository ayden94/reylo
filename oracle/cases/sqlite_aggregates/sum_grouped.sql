select customer, sum(cents) as cents, avg(weight) as weight from orders group by customer
