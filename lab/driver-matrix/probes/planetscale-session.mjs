// Control for the PlanetScale S8 failures: without motherload, concurrent execute() calls on one
// Connection that already holds a session are refused by ps-http-sim ("session already in use",
// HTTP 403). Run with ./setup.sh done: node probes/planetscale-session.mjs
import { connect } from "planetscale-1";
const conn = connect({ url: "http://root:porg@127.0.0.1:38080" });
await conn.execute("select 1");
const rs = await Promise.allSettled(Array.from({ length: 5 }, () => conn.execute("select id from people where id = ?", [1])));
console.log(rs.map((r) => (r.status === "fulfilled" ? "ok" : r.reason.message)));
