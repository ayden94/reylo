// dotsql decodes its own Postgres queries where the driver takes a decoder per query, so that a row
// has the type dotsql declares (decided 2026-10-03; the measurements are in the driver study of
// 2026-10-02). pg-types 2.x, which pg, pg-promise, Neon's Pool and Client and @vercel/postgres
// all decode through, returns `interval` and `interval[]` as objects, `numeric[]` as lossy floats,
// and every array it has no entry for (`name[]`, `"char"[]`, `xml[]`, range arrays, and every
// array whose oid the database assigns: enums, domains, extension types) as its unparsed text.
// dotsql declares an interval as its text, `numeric[]` as `string[]`, an enum array as a union
// array and a domain array as an array of its base type. So each such query carries a decoder
// table for that query only (`{ text, values, types }` on pg, `{ parsers }` on PGlite) that
// takes over exactly those oids while the driver still holds pg-types' default for them, which
// the default parsers' names tell (`noParse`, `PostgresInterval`, `parseIntervalArray`,
// `parseFloatArray`); every other oid, and one whose parser the user replaced, goes to the
// client's own table, so a parser the user registered still wins. An array element is decoded
// by the client's parser for its base type; a composite array stays text, as declared.
//
// An array's oid differs per database, so one catalog query per client (pool, connection or
// PGlite instance) reads every array type, cached in a WeakMap keyed by the user's object; a
// type created after that shows up as an oid the cache does not know, whose values come back as
// text, are looked up once, and are decoded before `run` returns. Nothing of the user's is
// mutated: not the client, not pg's global table, not a pool's options.
//
// Not decoded here: postgres.js and Bun.sql take no decoder per query; Neon's neon() function
// takes one, but replaces its whole table with it and exposes no table to fall back to; and a
// PGlite transaction object has no route to its instance's parsers.

type Parser = (text: string, oid?: number) => unknown;
type TypeTable = { getTypeParser(oid: number, format?: string): Parser };
type AnyClient = Record<string, any>;

/** One array type: elements decode as `base` (an element domain's base type); `text` keeps the whole array as text. */
type ArrayType = { elem: number; base: number; text: boolean; delim: string };
/** The array types of one database, and the other oids it knows, so that a new type is told apart. */
type Catalog = { arrays: Map<number, ArrayType>; known: Set<number> };

const catalogs = new WeakMap<object, Promise<Catalog>>();
/** A pool's fallback table: a client built from the pool's own class and options, never connected. */
const poolTables = new WeakMap<object, TypeTable>();
/** A pg-promise task's or database's fallback: the table of the connection the catalog query ran on. */
const pgpTables = new WeakMap<object, TypeTable | null>();

// Postgres gives every type a user creates an oid from 16384 up (FirstNormalObjectId); below
// that the built-in types, which the first catalog query already read.
const FIRST_USER_OID = 16384;

const INTERVAL = 1186;
const takenOver: Record<number, string> = { 1187: "parseIntervalArray", 1231: "parseFloatArray" };
const isDefaultInterval = (p: Parser) => p.name === "PostgresInterval" || p.name === "parseInterval";
const isNoParse = (p: Parser) => p.name === "noParse";

// Every array type but those of the system catalogs' row types, with the element's base type
// through any number of domains (a composite base marks the array as text). Joining on the
// element's typarray leaves out int2vector and oidvector, whose category is A but which print
// as `1 2`. An oid goes out as int8, since one above 2^31 would wrap as int4.
const catalogSql = (where: string) => `with recursive d(oid, base) as (
  select oid, typbasetype from pg_catalog.pg_type where typtype = 'd'
  union all
  select d.oid, t.typbasetype from d join pg_catalog.pg_type t on t.oid = d.base where t.typtype = 'd'
)
select e.typarray::int8 as "array", e.oid::int8 as elem, b.oid::int8 as base,
  b.typtype = 'c' or b.typcategory = 'A' as text, e.typdelim::text as delim
from pg_catalog.pg_type e
join pg_catalog.pg_type b on b.oid = coalesce((select d.base from d join pg_catalog.pg_type t on t.oid = d.base where d.oid = e.oid and t.typtype <> 'd'), e.oid)
where e.typarray <> 0 and ${where}`;
const allArrays = catalogSql(`(e.typtype <> 'c' or e.oid >= ${FIRST_USER_OID})`);
const someTypes = (oids: number[]) => {
  const list = `'{${oids.map((o) => Math.trunc(o)).join(",")}}'::pg_catalog.oid[]`;
  return catalogSql(`(e.typarray = any(${list}) or e.oid = any(${list}))`);
};

type Load = (sql: string) => Promise<Record<string, unknown>[]>;

function addRows(catalog: Catalog, rows: Record<string, unknown>[]) {
  for (const r of rows) {
    const array = Number(r.array);
    const elem = Number(r.elem);
    catalog.arrays.set(array, { elem, base: Number(r.base), text: r.text === true || r.text === "t", delim: String(r.delim ?? ",") });
    catalog.known.add(array).add(elem);
  }
}

function catalogFor(key: object, load: Load): Promise<Catalog> {
  let found = catalogs.get(key);
  if (!found) {
    found = load(allArrays).then((rows) => {
      const catalog: Catalog = { arrays: new Map(), known: new Set() };
      addRows(catalog, rows);
      return catalog;
    });
    // A failed lookup is not kept: the next query tries again.
    found.catch(() => catalogs.delete(key));
    catalogs.set(key, found);
  }
  return found;
}

/** Reads the types a result named that the catalog did not know, once each. */
async function learn(catalog: Catalog, oids: Iterable<number>, load: Load) {
  const missing = [...new Set(oids)].filter((oid) => !catalog.known.has(oid));
  if (!missing.length) return;
  addRows(catalog, await load(someTypes(missing)));
  for (const oid of missing) catalog.known.add(oid);
}

/**
 * Postgres's array output: `{a,"b,c",NULL,{d}}`, optionally after its bounds (`[0:1]={...}`).
 * An element is quoted when it holds the delimiter, a brace, a quote, a backslash or white space,
 * or is empty or the word NULL; inside quotes a backslash escapes the next character.
 */
export function parseArray(text: string, element: (text: string) => unknown, delim = ","): unknown[] {
  const n = text.length;
  let i = text[0] === "[" ? text.indexOf("=") + 1 : 0;
  const level = (): unknown[] => {
    const out: unknown[] = [];
    i++;
    if (text[i] === "}") return i++, out;
    while (i < n) {
      if (text[i] === "{") out.push(level());
      else if (text[i] === '"') {
        let s = "";
        let from = ++i;
        while (i < n && text[i] !== '"') {
          if (text[i] === "\\") (s += text.slice(from, i)), (from = ++i);
          i++;
        }
        out.push(element(s + text.slice(from, i++)));
      } else {
        const from = i;
        while (i < n && text[i] !== delim && text[i] !== "}") i++;
        const s = text.slice(from, i);
        out.push(s === "NULL" ? null : element(s));
      }
      if (text[i++] === "}") return out;
    }
    return out;
  };
  return level();
}

const asText = (text: string) => text;

/** The text of a value whose type the catalog did not know yet; replaced before `run` returns. */
class Pending {
  readonly oid: number;
  readonly text: string;
  constructor(oid: number, text: string) {
    this.oid = oid;
    this.text = text;
  }
}

/**
 * The decoder table for one query on a pg-family client: dotsql's parser for the oids it takes over
 * while `fallback` still holds pg-types' default for them, `fallback`'s own parser for every other.
 */
function pgTypes(catalog: Catalog, fallback: TypeTable, pending: Set<number>): TypeTable {
  const own = (oid: number, format?: string): Parser => fallback.getTypeParser(oid, format ?? "text");
  const scalar = (oid: number): Parser => {
    const p = own(oid);
    return oid === INTERVAL && isDefaultInterval(p) ? asText : p;
  };
  const getTypeParser = (oid: number, format?: string): Parser => {
    const p = own(oid, format);
    if (format && format !== "text") return p;
    const array = catalog.arrays.get(oid);
    if (array) {
      if (array.text || !(isNoParse(p) || takenOver[oid] === p.name)) return p;
      const element = scalar(array.base);
      return (text) => parseArray(text, element, array.delim);
    }
    if (oid >= FIRST_USER_OID && !catalog.known.has(oid) && isNoParse(p)) {
      pending.add(oid);
      return (text) => new Pending(oid, text);
    }
    return oid === INTERVAL && isDefaultInterval(p) ? asText : p;
  };
  return { getTypeParser };
}

/** Looks up the types that came back as Pending, and decodes their values in place. */
async function settle(rows: unknown, catalog: Catalog, fallback: TypeTable, pending: Set<number>, load: Load) {
  if (!pending.size || !Array.isArray(rows)) return;
  try {
    await learn(catalog, pending, load);
  } catch {
    // The values stay their text, which is what the driver would have returned.
  }
  const table = pgTypes(catalog, fallback, new Set());
  for (const row of rows) {
    if (row === null || typeof row !== "object") continue;
    for (const [key, value] of Object.entries(row)) {
      if (value instanceof Pending) (row as Record<string, unknown>)[key] = catalog.known.has(value.oid) ? table.getTypeParser(value.oid)(value.text) : value.text;
    }
  }
}

const rowsOf = (result: any): unknown[] => (Array.isArray(result) ? result : result.rows);

/**
 * Runs a Postgres statement on a client whose query() dotsql calls: pg's Client, PoolClient and
 * Pool, Neon's, pg-promise's database, tasks and transactions, PGlite, and Neon's neon(), the
 * last unchanged. Each client is told apart by what it has; anything else is called as before.
 */
export async function queryPostgres(c: AnyClient, text: string, values: unknown[]): Promise<unknown[]> {
  // pg-promise formats the values into the text itself (reading $1 inside a string literal
  // too) unless it is handed a { text, values } object, which it binds.
  if (typeof c.manyOrNone === "function") return pgPromise(c, text, values);
  // pg's Client and PoolClient, Neon's, and @vercel/postgres's VercelClient.
  if (typeof c.getTypeParser === "function") return pgClient(c, c, text, values);
  // PGlite: an instance keeps its parsers in `parsers` and re-reads its arrays on request.
  if (typeof c.refreshArrayTypes === "function" && c.parsers && typeof c.parsers === "object") return pglite(c, text, values);
  // pg's Pool, Neon's and @vercel/postgres's: query() checks a client out of the pool.
  if (typeof c.connect === "function" && typeof c.Client === "function") return pgPool(c, text, values);
  return rowsOf(await c.query(text, values));
}

async function pgClient(key: object, c: AnyClient, text: string, values: unknown[]): Promise<unknown[]> {
  const load: Load = async (sql) => (await c.query(sql)).rows;
  const catalog = await catalogFor(key, load);
  const fallback: TypeTable = { getTypeParser: (oid, format) => c.getTypeParser(oid, format) };
  const pending = new Set<number>();
  const rows = rowsOf(await c.query({ text, values, types: pgTypes(catalog, fallback, pending) }));
  await settle(rows, catalog, fallback, pending, load);
  return rows;
}

// A pool has no getTypeParser. Its clients' tables are the pool's `types` option over pg's
// global table, plus whatever a `connect` or `acquire` listener sets on each client. With no
// such listener a client built from the pool's class and options, never connected, has the
// same table, and pool.query keeps its own behaviour (Neon's goes over HTTP only while the pool
// has no listener). With one, the query runs on a client checked out the way pool.query does,
// whose own table then holds what the listener set.
async function pgPool(pool: AnyClient, text: string, values: unknown[]): Promise<unknown[]> {
  const listened = typeof pool.listenerCount !== "function" || pool.listenerCount("connect") > 0 || pool.listenerCount("acquire") > 0;
  if (listened) return checkedOut(pool, (client) => pgClient(pool, client, text, values));
  let fallback = poolTables.get(pool);
  if (!fallback) poolTables.set(pool, (fallback = new pool.Client(pool.options) as TypeTable));
  const table = fallback;
  const load: Load = async (sql) => (await pool.query(sql)).rows;
  const catalog = await catalogFor(pool, load);
  const pending = new Set<number>();
  const rows = rowsOf(await pool.query({ text, values, types: pgTypes(catalog, table, pending) }));
  await settle(rows, catalog, table, pending, load);
  return rows;
}

/** pg-pool's query(), around more than one statement: an error releases the client with it, which ends it. */
async function checkedOut<T>(pool: AnyClient, work: (client: AnyClient) => Promise<T>): Promise<T> {
  const client: AnyClient = await pool.connect();
  let released = false;
  const release = (error?: unknown) => {
    if (!released) (released = true), client.release(error);
  };
  let onError: (error: Error) => void = () => {};
  const lost = new Promise<never>((_, reject) => (onError = (error) => (release(error), reject(error))));
  lost.catch(() => {});
  client.once?.("error", onError);
  try {
    const result = await Promise.race([work(client), lost]);
    release();
    return result;
  } catch (error) {
    release(error);
    throw error;
  } finally {
    client.removeListener?.("error", onError);
  }
}

// pg-promise sends a { text, values, types } object to pg as it is. Its database, tasks and
// transactions expose no connection, so the fallback is the table of the connection the catalog
// query ran on, read from pg's Result (`_types`: the connection's own parsers, the pool's
// `types`, pg's global table); a task runs on one connection, and a database's connections share
// the pool's options and pg-promise's `connect` handler. Where it is missing, the query runs as before.
async function pgPromise(db: AnyClient, text: string, values: unknown[]): Promise<unknown[]> {
  if (typeof db.result !== "function") return rowsOf(await db.query({ text, values }));
  const load: Load = async (sql) => {
    const result = await db.result(sql);
    if (!pgpTables.has(db)) pgpTables.set(db, typeof result?._types?.getTypeParser === "function" ? result._types : null);
    return result.rows;
  };
  const catalog = await catalogFor(db, load);
  const fallback = pgpTables.get(db);
  if (!fallback) return rowsOf(await db.query({ text, values }));
  const pending = new Set<number>();
  const rows = rowsOf(await db.query({ text, values, types: pgTypes(catalog, fallback, pending) }));
  await settle(rows, catalog, fallback, pending, load);
  return rows;
}

// PGlite merges a query's `parsers` over the instance's, and builds an array parser at start for
// every array type then in the database, which applies the element's parser: a domain's elements
// have none and stay text, a composite's become a string[], and a type created later stays text
// until refreshArrayTypes(). dotsql decodes those three, through the instance's parser for the
// base type, and leaves an array alone whose element has a parser of the user's.
async function pglite(db: AnyClient, text: string, values: unknown[]): Promise<unknown[]> {
  const load: Load = async (sql) => (await db.query(sql)).rows;
  const catalog = await catalogFor(db, load);
  const result = await db.query(text, values, { parsers: pgliteParsers(catalog, db.parsers) });
  const fields: { name: string; dataTypeID: number }[] = result?.fields ?? [];
  const raw = fields.filter((f) => f.dataTypeID >= FIRST_USER_OID && !catalog.known.has(f.dataTypeID) && !db.parsers[f.dataTypeID]);
  if (raw.length) {
    try {
      await learn(catalog, raw.map((f) => f.dataTypeID), load);
    } catch {
      return result.rows;
    }
    const parsers = pgliteParsers(catalog, db.parsers);
    for (const f of raw) {
      const parse = parsers[f.dataTypeID];
      if (!parse) continue;
      for (const row of result.rows) if (typeof row[f.name] === "string") row[f.name] = parse(row[f.name], f.dataTypeID);
    }
  }
  return result.rows;
}

function pgliteParsers(catalog: Catalog, instance: Record<number, Parser | undefined>): Record<number, Parser> {
  const parsers: Record<number, Parser> = {};
  for (const [oid, array] of catalog.arrays) {
    const own = instance[array.elem];
    if (array.text) {
      if (instance[oid] && !own) parsers[oid] = asText;
      continue;
    }
    if (instance[oid] && (own || array.elem === array.base)) continue;
    const base = instance[array.base];
    const element = own ? (s: string) => own(s, array.elem) : base ? (s: string) => base(s, array.base) : asText;
    parsers[oid] = (s) => parseArray(s, element, array.delim);
  }
  return parsers;
}
