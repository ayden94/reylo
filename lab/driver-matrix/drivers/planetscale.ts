// @planetscale/database through ps-http-sim (porg-dm-ps-sim, PlanetScale's HTTP API in front of
// the MySQL 8.4 container): a connect() Connection, a Client, and conn.transaction(tx => ...).
import { args, callbackTx, notRun, rawSavepoint, requireFrom, runJob } from "../lib/harness.ts";

const a = args();
const meta = { pkg: "@planetscale/database", version: a.version!, target: "MySQL 8.4 via ps-http-sim" };
const url = `http://root:porg@${process.env.DM_PS_SIM ?? "127.0.0.1:38080"}`;

let ps: any;
try {
  ps = requireFrom(a.pkg!);
} catch (e) {
  notRun(meta, `cannot load ${a.pkg}: ${(e as Error).message}`);
  process.exit(0);
}

await runJob(meta, async () => {
  const conn = ps.connect({ url });
  const client = new ps.Client({ url });
  const begin = (fn: (tx: any) => Promise<unknown>) => conn.transaction(fn);
  // 2.0 takes no query parameters at all, so the raw count inlines the tag as a literal.
  const literal = (s: string) => `'${s.replace(/'/g, "''")}'`;
  // 2.0 binds no parameters by design; motherload refuses a statement with parameters on it, with its own
  // TypeError whose cause is PlanetScale's error, and runs a statement without them.
  const paramsRefused = a.version!.startsWith("2.") ? /binds no parameters/ : undefined;
  return {
    ...meta,
    dialect: "mysql",
    count: async (tag) => Number((await conn.execute(`select count(*) as n from scratch where tag = ${literal(tag)}`)).rows[0].n),
    shapes: [
      { name: "connect() Connection", kind: "async", scope: (fn) => fn(conn), paramsRefused },
      { name: "Client", kind: "async", scope: (fn) => fn(client), paramsRefused },
      {
        name: "conn.transaction(tx)",
        kind: "async",
        inTransaction: true,
        paramsRefused,
        scope: (fn) => begin((tx) => fn(tx)),
        tx: callbackTx(
          begin,
          rawSavepoint((tx, sql) => tx.execute(sql)),
        ),
      },
    ],
  };
});
