// node-postgres (pg) against the shared local Postgres: Pool, a connected Client, a checked-out
// PoolClient, and a PoolClient inside BEGIN/COMMIT.
import { args, lazy, manualTx, notRun, requireFrom, runJob } from "../lib/harness.ts";

const a = args();
const meta = { pkg: "pg", version: a.version!, target: "Postgres 14 (local)" };
const url = process.env.DM_PG_URL ?? "postgres://localhost:5432/postgres";

let pg: any;
try {
  pg = requireFrom(a.pkg!);
} catch (e) {
  notRun(meta, `cannot load ${a.pkg}: ${(e as Error).message}`);
  process.exit(0);
}

await runJob(meta, async () => {
  const pool = new pg.Pool({ connectionString: url, max: 10, connectionTimeoutMillis: 10000 });
  const client = lazy(async () => {
    const c = new pg.Client({ connectionString: url });
    await c.connect();
    return c;
  });
  const checkout = async () => {
    const c = await pool.connect();
    return { client: c, release: () => c.release() };
  };
  const manual = {
    open: checkout,
    begin: (c: any) => c.query("BEGIN"),
    commit: (c: any) => c.query("COMMIT"),
    rollback: (c: any) => c.query("ROLLBACK"),
    exec: (c: any, sql: string) => c.query(sql),
  };
  return {
    ...meta,
    dialect: "postgres",
    count: async (tag) => (await pool.query("select count(*)::int as n from driver_matrix.scratch where tag = $1", [tag])).rows[0].n,
    shapes: [
      { name: "Pool", kind: "async", scope: (fn) => fn(pool) },
      { name: "Client (connected)", kind: "async", scope: async (fn) => fn(await client()) },
      {
        name: "PoolClient (pool.connect())",
        kind: "async",
        scope: async (fn) => {
          const { client: c, release } = await checkout();
          try {
            await fn(c);
          } finally {
            release();
          }
        },
      },
      {
        name: "PoolClient in BEGIN/COMMIT",
        kind: "async",
        inTransaction: true,
        scope: async (fn) => {
          const { client: c, release } = await checkout();
          try {
            await c.query("BEGIN");
            await fn(c);
            await c.query("COMMIT");
          } catch (e) {
            await c.query("ROLLBACK").catch(() => {});
            throw e;
          } finally {
            release();
          }
        },
        tx: manualTx(manual),
      },
    ],
    close: async () => {
      await pool.end();
      await (await client()).end();
    },
  };
});
