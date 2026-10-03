import postgres from "postgres";
import query from "./query.sql";
// dotsql declares what the driver returns and converts nothing, so the driver builds the values
// `.dotsql.yaml` names: Temporal for timestamptz, timestamp and date (and their arrays, which
// postgres parses element by element), and a BigInt for int8.
const temporal = (oid: number, from: (text: string) => unknown) => ({ to: oid, from: [oid], serialize: String, parse: from });
const sql = postgres("postgres://localhost:5432/postgres", {
  types: {
    bigint: postgres.BigInt,
    instant: temporal(1184, (t) => Temporal.Instant.from(t.replace(" ", "T").replace(/([+-]\d\d)$/, "$1:00"))),
    plainDateTime: temporal(1114, (t) => Temporal.PlainDateTime.from(t.replace(" ", "T"))),
    plainDate: temporal(1082, (t) => Temporal.PlainDate.from(t)),
  },
});
const ada = await query.run(sql, { id: 1 });
const kind = (v: unknown) => (typeof v === "object" && v !== null ? v.constructor.name : typeof v);
// JSON.stringify has no rule for a BigInt; Temporal values serialize through their toJSON.
console.log(JSON.stringify({
  kinds: ada && Object.fromEntries(Object.entries(ada).map(([k, v]) => [k, Array.isArray(v) ? `${kind(v[0])}[]` : kind(v)])),
  values: ada,
}, (_, v) => (typeof v === "bigint" ? `${v}n` : v)));
await sql.end();
