fn main() {
    let sql = "SELECT u.id, u.email, jsonb_build_object('id', p.id, 'title', p.title) AS post FROM users u JOIN posts p ON p.user_id = u.id WHERE u.id = $1 AND p.published_at > now() - interval '7 days' ORDER BY p.published_at DESC LIMIT 20";
    let t = std::time::Instant::now();
    let r = pg_query::parse(sql).expect("parse");
    let first = t.elapsed();
    let n = 2000;
    let t = std::time::Instant::now();
    for _ in 0..n { let _ = pg_query::parse(sql).unwrap(); }
    let per = t.elapsed().as_secs_f64() / n as f64 * 1e6;
    let dep = pg_query::deparse(&r.protobuf).expect("deparse");
    let t = std::time::Instant::now();
    for _ in 0..n { let _ = pg_query::deparse(&r.protobuf).unwrap(); }
    let dper = t.elapsed().as_secs_f64() / n as f64 * 1e6;
    // error path exercises PG_TRY/siglongjmp
    let err = pg_query::parse("SELEC 1").err().map(|e| e.to_string());
    println!("first_parse_us={} warm_parse_us={:.1} warm_deparse_us={:.1} stmts={} deparse_len={} err={:?}", first.as_micros(), per, dper, r.protobuf.stmts.len(), dep.len(), err);
}
