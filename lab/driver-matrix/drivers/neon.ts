// @neondatabase/serverless through a local Neon proxy (porg-dm-neon-proxy, in front of the
// porg-dm-neon-pg container): Pool, Client and PoolClient over WebSocket, and the neon() HTTP
// function, with and without fullResults.
import ws from "ws";
import { args, lazy, manualTx, notRun, requireFrom, runJob } from "../lib/harness.ts";

const a = args();
const meta = { pkg: "@neondatabase/serverless", version: a.version!, target: "Postgres 14 via local Neon proxy" };
// The proxy knows the endpoint by this host name; nothing resolves it, because fetchEndpoint and
// wsProxy send every request to the proxy's local port.
const url = "postgres://postgres:postgres@db.localtest.me:5432/main";
const proxy = process.env.DM_NEON_PROXY ?? "127.0.0.1:54444";

let neon: any;
try {
  neon = requireFrom(a.pkg!);
} catch (e) {
  notRun(meta, `cannot load ${a.pkg}: ${(e as Error).message}`);
  process.exit(0);
}

await runJob(meta, async () => {
  const { neonConfig, Pool, Client } = neon;
  neonConfig.fetchEndpoint = () => `http://${proxy}/sql`;
  neonConfig.useSecureWebSocket = false;
  neonConfig.wsProxy = () => `${proxy}/v1`;
  neonConfig.webSocketConstructor = ws;
  const pool = new Pool({ connectionString: url, max: 10 });
  const client = lazy(async () => {
    const c = new Client(url);
    await c.connect();
    return c;
  });
  const http = neon.neon(url);
  const httpFull = neon.neon(url, { fullResults: true });
  const checkout = async () => {
    const c = await pool.connect();
    return { client: c, release: () => c.release() };
  };
  const noTx = "neon()'s transaction() takes an array of queries built by the sql function, not a client, so a porg query cannot join it";
  return {
    ...meta,
    dialect: "postgres",
    count: async (tag) => (await pool.query("select count(*)::int as n from driver_matrix.scratch where tag = $1", [tag])).rows[0].n,
    shapes: [
      { name: "Pool (WebSocket)", kind: "async", scope: (fn) => fn(pool) },
      { name: "Client (WebSocket, connected)", kind: "async", scope: async (fn) => fn(await client()) },
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
        tx: manualTx({
          open: checkout,
          begin: (c) => c.query("BEGIN"),
          commit: (c) => c.query("COMMIT"),
          rollback: (c) => c.query("ROLLBACK"),
          exec: (c, sql) => c.query(sql),
        }),
      },
      { name: "neon() HTTP function", kind: "async", scope: (fn) => fn(http), skip: { S7: noTx } },
      { name: "neon(url, { fullResults: true })", kind: "async", scope: (fn) => fn(httpFull) },
    ],
    close: async () => {
      await pool.end();
      await (await client()).end();
    },
  };
});
