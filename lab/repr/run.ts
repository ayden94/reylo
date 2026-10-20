import postgres from "postgres";
import query from "./query.sql";
// The app sets up postgres.js to build Temporal values for timestamptz, timestamp and date (and
// their arrays, which postgres parses element by element) and a BigInt for int8. Its own queries
// get those; Reylo's gets its one representation, since it reads each value's text itself
// (`.raw()`) and never the app's parsers.
const temporal = (oid: number, from: (text: string) => unknown) => ({ to: oid, from: [oid], serialize: String, parse: from });
const sql = postgres(process.env["DATABASE_URL"] ?? "postgres://localhost:5432/postgres", {
  types: {
    bigint: postgres.BigInt,
    instant: temporal(1184, (t) => Temporal.Instant.from(t.replace(" ", "T").replace(/([+-]\d\d)$/, "$1:00"))),
    plainDateTime: temporal(1114, (t) => Temporal.PlainDateTime.from(t.replace(" ", "T"))),
    plainDate: temporal(1082, (t) => Temporal.PlainDate.from(t)),
  },
  onnotice: () => {},
});
const kind = (v: unknown) => (typeof v === "object" && v !== null ? v.constructor.name : typeof v);
const kinds = (row: Record<string, unknown> | null | undefined) => row && Object.fromEntries(Object.entries(row).map(([k, v]) => [k, Array.isArray(v) ? `${kind(v[0])}[]` : kind(v)]));
const reylo = await query.run(sql, { id: 1 });
// The app runs the same statement itself: `query.sql` is the text Reylo sends, with `$1`.
const [app] = await sql.unsafe(query.sql, [1]);
console.log(JSON.stringify({ reylo: kinds(reylo), app: kinds(app), values: reylo }));
await sql.end();
