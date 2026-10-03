// The JavaScript half of analysis: run Parse/Describe against the project's own
// Postgres client. Drivers are resolved from the PROJECT directory, never from
// dotsql's own dependencies, so every driver stays an optional peer.
import { createRequire } from "node:module";
import { Unreachable } from "./engine.ts";

export type Described = {
  params: { oid: number }[];
  columns: { name: string; oid: number; table: number; attnum: number }[];
};

export interface Describer {
  readonly driver: "postgres" | "pg";
  describe(sql: string): Promise<Described>;
  query(sql: string, params?: unknown[]): Promise<Record<string, unknown>[]>;
  /** Rows as arrays of values, in column order: what the engine's catalog queries read. */
  values(sql: string, params?: unknown[]): Promise<unknown[][]>;
  close(): Promise<void>;
}

/**
 * The server's answer to a statement carries a severity (pg's DatabaseError, postgres.js's
 * PostgresError). Anything else is the connection: a socket error, a timeout, a closed client.
 * So is a FATAL or PANIC error and the connection-exception class 08, after which the server
 * has closed the session (a database that does not exist is a FATAL 3D000).
 */
function settled(e: unknown): Error {
  const err = e as { severity?: string; code?: string };
  return err?.severity && err.severity !== "FATAL" && err.severity !== "PANIC" && !err.code?.startsWith("08") ? (e as Error) : new Unreachable(e);
}

/** How long a connection attempt may take: the transform waits on it, in the editor and in CI. */
const CONNECT_TIMEOUT_SECONDS = 10;

/**
 * postgres.js sends a URL parameter it does not know to the server as a setting, and the server
 * refuses `channel_binding`, which Neon's URLs carry. postgres.js cannot bind the channel either,
 * so a URL that requires it has the server's certificate verified instead.
 */
export function forPostgresJs(url: string): { url: string; ssl?: "verify-full" } {
  const q = url.indexOf("?");
  if (q < 0) return { url };
  const params = new URLSearchParams(url.slice(q + 1));
  const binding = params.get("channel_binding");
  if (binding === null) return { url };
  params.delete("channel_binding");
  const rest = params.toString();
  const stripped = url.slice(0, q) + (rest ? `?${rest}` : "");
  return binding === "require" ? { url: stripped, ssl: "verify-full" } : { url: stripped };
}

export async function openPostgresDescriber(projectDir: string, url: string): Promise<Describer> {
  const require = createRequire(projectDir.replace(/\/?$/, "/package.json"));
  const resolve = (name: string) => { try { return require.resolve(name); } catch { return undefined; } };
  const rethrow = (e: unknown): never => { throw settled(e); };

  const porsager = resolve("postgres");
  if (porsager) {
    const postgres = (await import(porsager)).default as (url: string, o?: object) => any;
    const target = forPostgresJs(url);
    const sql = postgres(target.url, { max: 1, prepare: false, onnotice: () => {}, connect_timeout: CONNECT_TIMEOUT_SECONDS, ...(target.ssl && { ssl: target.ssl }) });
    // postgres.js runs these queries one at a time and connects again for each one queued behind
    // a failure, so five transforms against a host that never answers would wait five timeouts
    // (ending the client does not release the queue). The first failure answers every call that
    // is waiting and every call after it.
    let failure: Error | undefined;
    let abort!: (e: Error) => void;
    const aborted = new Promise<never>((_, reject) => { abort = reject; });
    aborted.catch(() => {});
    const guard = <T>(run: () => PromiseLike<T>): Promise<T> => {
      if (failure) return Promise.reject(failure);
      return Promise.race([run(), aborted]).catch((e: unknown): never => {
        if (failure) throw failure;
        const err = settled(e);
        if (err instanceof Unreachable) {
          failure = err;
          abort(err);
          sql.end({ timeout: 0 }).catch(() => {});
        }
        throw err;
      });
    };
    return {
      driver: "postgres",
      async describe(text) {
        const d = await guard<any>(() => sql.unsafe(text).describe());
        return {
          params: (d.types as number[]).map((oid) => ({ oid })),
          columns: (d.columns as any[]).map((c) => ({ name: c.name, oid: c.type, table: c.table, attnum: c.number })),
        };
      },
      query: (text, params = []) => guard(() => sql.unsafe(text, params as any[])),
      values: (text, params = []) => guard(() => sql.unsafe(text, params as any[]).values()),
      close: () => sql.end({ timeout: 0 }),
    };
  }

  const pgPath = resolve("pg");
  if (pgPath) {
    const pg = (await import(pgPath)).default as any;
    const client = new pg.Client({ connectionString: url, connectionTimeoutMillis: CONNECT_TIMEOUT_SECONDS * 1000 });
    // A connection that drops while idle is reported as an event; unheard, it ends the process.
    // The next call fails on the closed client and reopens it.
    client.on("error", () => {});
    await client.connect().catch(rethrow);
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
            handleError(err: Error) { cleanup(); reject(settled(err)); },
            handleReadyForQuery() { cleanup(); resolve(out); },
          };
          client.query(submittable);
        });
      },
      query: async (text, params = []) => (await client.query(text, params).catch(rethrow)).rows,
      values: async (text, params = []) => (await client.query({ text, values: params, rowMode: "array" }).catch(rethrow)).rows,
      close: () => client.end(),
    };
  }
  throw new Error(`No Postgres client found in ${projectDir}; install "postgres" or "pg".`);
}
