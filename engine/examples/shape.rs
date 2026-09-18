fn main() {
    let sql = std::env::args().nth(1).unwrap();
    let p = pg_query::parse(&sql).unwrap();
    println!("{}", serde_json::to_string(&p.protobuf).unwrap());
}
