select coalesce(maybe, 0) as zero, max(done) as most, maybe + 0 as plus, if(done, maybe, maybe) as either from flags group by done, maybe
