// Cloudflare D1 through Miniflare (a local workerd; the binding is reached from Node through
// Miniflare's proxy): the D1Database and a withSession() session. D1 has no interactive
// transaction (batch() takes prepared statements, not a client), so there is no S7.
import { args, notRun, requireFrom, runJob, seedSql, statements } from "../lib/harness.ts";

const a = args();
const meta = { pkg: "miniflare D1", version: a.version!, target: "D1 (Miniflare, local workerd)" };

let Miniflare: any;
try {
  Miniflare = requireFrom(a.pkg ?? "miniflare-4").Miniflare;
} catch (e) {
  notRun(meta, `cannot load miniflare: ${(e as Error).message.split("\n")[0]}`);
  process.exit(0);
}

let mf: any;
await runJob(meta, async () => {
  mf = new Miniflare({ modules: true, script: "export default {}", d1Databases: ["DB"] });
  const db = await mf.getD1Database("DB");
  await db.batch(statements(seedSql("sqlite")).map((s) => db.prepare(s)));
  const noTx = "D1 has no interactive transaction: batch() takes prepared statements, not a client";
  const shapes: any[] = [{ name: "D1Database", kind: "async", scope: (fn: any) => fn(db), skip: { S7: noTx } }];
  if (typeof db.withSession === "function") shapes.push({ name: "db.withSession()", kind: "async", scope: (fn: any) => fn(db.withSession()) });
  return {
    ...meta,
    dialect: "sqlite",
    count: async (tag) => (await db.prepare("select count(*) as n from scratch where tag = ?").bind(tag).first()).n,
    shapes,
    close: () => mf.dispose(),
  };
});
