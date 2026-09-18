// The JavaScript half of analysis: run Parse/Describe against the project's own
// Postgres client. Drivers are resolved from the PROJECT directory, never from
// lando's own dependencies, so every driver stays an optional peer.
import { createRequire } from "node:module";

export type Described = {
  params: { oid: number }[];
  columns: { name: string; oid: number; table: number; attnum: number }[];
};

export interface Describer {
  readonly driver: "postgres" | "pg";
  describe(sql: string): Promise<Described>;
  query(sql: string, params?: unknown[]): Promise<Record<string, unknown>[]>;
  close(): Promise<void>;
}

export async function openPostgresDescriber(projectDir: string, url: string): Promise<Describer> {
  const require = createRequire(projectDir.replace(/\/?$/, "/package.json"));
  const resolve = (name: string) => { try { return require.resolve(name); } catch { return undefined; } };

  const porsager = resolve("postgres");
  if (porsager) {
    const postgres = (await import(porsager)).default as (url: string, o?: object) => any;
    const sql = postgres(url, { max: 1, prepare: false, onnotice: () => {} });
    return {
      driver: "postgres",
      async describe(text) {
        const d = await sql.unsafe(text).describe();
        return {
          params: (d.types as number[]).map((oid) => ({ oid })),
          columns: (d.columns as any[]).map((c) => ({ name: c.name, oid: c.type, table: c.table, attnum: c.number })),
        };
      },
      query: (text, params = []) => sql.unsafe(text, params as any[]),
      close: () => sql.end(),
    };
  }

  const pgPath = resolve("pg");
  if (pgPath) {
    const pg = (await import(pgPath)).default as any;
    const client = new pg.Client({ connectionString: url });
    await client.connect();
    return {
      driver: "pg",
      // node-postgres has no public describe, but it accepts any "submittable"
      // object in client.query (pg-cursor works this way). This one sends
      // Parse + Describe + Sync and collects the descriptions; parameterDescription
      // and noData are not routed to the query by pg, so they are read off the
      // connection while the query is active.
      describe(text) {
        return new Promise<Described>((resolve, reject) => {
          const out: Described = { params: [], columns: [] };
          let con: any;
          const cleanup = () => { con?.off("parameterDescription", onParam); con?.off("noData", onNoData); };
          const onParam = (m: any) => { out.params = (m.dataTypeIDs as number[]).map((oid) => ({ oid })); };
          const onNoData = () => { out.columns = []; };
          const submittable = {
            submit(connection: any) {
              con = connection;
              con.on("parameterDescription", onParam);
              con.on("noData", onNoData);
              con.parse({ name: "", text, types: [] });
              con.describe({ type: "S", name: "" });
              con.sync();
            },
            handleRowDescription(m: any) {
              out.columns = (m.fields as any[]).map((f) => ({ name: f.name, oid: f.dataTypeID, table: f.tableID, attnum: f.columnID }));
            },
            handleParseComplete() {},
            handleDataRow() {},
            handleCommandComplete() {},
            handleEmptyQuery() {},
            handlePortalSuspended() {},
            // pg drops the active query on an error message before ReadyForQuery,
            // so the rejection cannot wait for handleReadyForQuery.
            handleError(err: Error) { cleanup(); reject(err); },
            handleReadyForQuery() { cleanup(); resolve(out); },
          };
          client.query(submittable);
        });
      },
      query: async (text, params = []) => (await client.query(text, params)).rows,
      close: () => client.end(),
    };
  }
  throw new Error(`No Postgres client found in ${projectDir}; install "postgres" or "pg".`);
}
