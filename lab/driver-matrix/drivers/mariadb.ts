// The mariadb connector against a MariaDB or MySQL container (--port, --target): Pool,
// Connection, PoolConnection, a PoolConnection inside beginTransaction(), and the callback API's
// Connection (mariadb/callback).
import { args, lazy, manualTx, notRun, requireFrom, runJob } from "../lib/harness.ts";

const a = args();
const meta = { pkg: "mariadb", version: a.version!, target: a.target! };
// MySQL 8.4's caching_sha2_password needs the server's RSA key when the connection has no TLS.
const config = { host: "127.0.0.1", port: Number(a.port), user: "root", password: "dotsql", database: "driver_matrix", allowPublicKeyRetrieval: true };

let mariadb: any;
let callbackApi: any;
try {
  mariadb = requireFrom(a.pkg!);
  callbackApi = requireFrom(`${a.pkg}/callback`);
} catch (e) {
  notRun(meta, `cannot load ${a.pkg}: ${(e as Error).message}`);
  process.exit(0);
}

await runJob(meta, async () => {
  const pool = mariadb.createPool({ ...config, connectionLimit: 10 });
  const conn = lazy(() => mariadb.createConnection(config));
  const cbConn = lazy(
    () =>
      new Promise((resolve, reject) => {
        const c = callbackApi.createConnection(config);
        c.connect((e: unknown) => (e ? reject(e) : resolve(c)));
      }),
  );
  const checkout = async () => {
    const c = await pool.getConnection();
    return { client: c, release: () => c.release() };
  };
  const cbPool = callbackApi.createPool({ ...config, connectionLimit: 5 });
  const cb = (fn: (done: (e: unknown, v?: unknown) => void) => void) => new Promise<any>((resolve, reject) => fn((e, v) => (e ? reject(e) : resolve(v))));
  const cbCheckout = async () => {
    const c = await cb((done) => cbPool.getConnection(done));
    return { client: c, release: () => c.release() };
  };
  return {
    ...meta,
    dialect: "mysql",
    count: async (tag) => Number((await pool.query("select count(*) as n from scratch where tag = ?", [tag]))[0].n),
    shapes: [
      { name: "Pool", kind: "async", scope: (fn) => fn(pool) },
      { name: "Connection", kind: "async", scope: async (fn) => fn(await conn()) },
      {
        name: "PoolConnection (getConnection())",
        kind: "async",
        scope: async (fn) => {
          const { client, release } = await checkout();
          try {
            await fn(client);
          } finally {
            await release();
          }
        },
      },
      {
        name: "PoolConnection in beginTransaction()",
        kind: "async",
        inTransaction: true,
        scope: async (fn) => {
          const { client, release } = await checkout();
          try {
            await client.beginTransaction();
            await fn(client);
            await client.commit();
          } catch (e) {
            await client.rollback().catch(() => {});
            throw e;
          } finally {
            await release();
          }
        },
        tx: manualTx({
          open: checkout,
          begin: (c) => c.beginTransaction(),
          commit: (c) => c.commit(),
          rollback: (c) => c.rollback(),
          exec: (c, sql) => c.query(sql),
        }),
      },
      { name: "callback Connection (mariadb/callback)", kind: "async", scope: async (fn) => fn(await cbConn()) },
      { name: "callback Pool (mariadb/callback)", kind: "async", scope: (fn) => fn(cbPool) },
      {
        name: "callback PoolConnection in beginTransaction() (mariadb/callback)",
        kind: "async",
        inTransaction: true,
        scope: async (fn) => {
          const { client, release } = await cbCheckout();
          try {
            await cb((done) => client.beginTransaction(done));
            await fn(client);
            await cb((done) => client.commit(done));
          } catch (e) {
            await cb((done) => client.rollback(done)).catch(() => {});
            throw e;
          } finally {
            release();
          }
        },
        tx: manualTx({
          open: cbCheckout,
          begin: (c) => cb((done) => c.beginTransaction(done)),
          commit: (c) => cb((done) => c.commit(done)),
          rollback: (c) => cb((done) => c.rollback(done)),
          exec: (c, sql) => cb((done) => c.query(sql, done)),
        }),
      },
    ],
    close: async () => {
      await pool.end();
      await (await conn()).end();
      ((await cbConn()) as any).end(() => {});
      await cb((done) => cbPool.end(done));
    },
  };
});
