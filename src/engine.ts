// The analysis engine: `engine/reylo_engine.wasm`, a WASI reactor, loaded once per process.
// The engine is pure. `analyze` replays it: every round it either answers or names what it
// still needs — statements to describe, catalog queries to run — and this module fulfils
// those through the project's own driver and calls again.
import { readFileSync } from "node:fs";
import type { WASI as WasiClass } from "node:wasi";
import type { EngineType } from "./engine-type.ts";

export type { EngineType } from "./engine-type.ts";
export type Dialect = "postgres" | "sqlite" | "mysql";

export type Diagnostic = { message: string; start: number; length: number; code: number };

export type Analysis = {
  kind: string;
  /** `nullable`: the statement lets the caller pass null (it tests the parameter for null, coalesces it, or writes it whole to a nullable column). */
  params: { name: string; type: EngineType; type_name: string; nullable: boolean }[];
  columns: { name: string; type: EngineType; nullable: boolean; type_name: string }[];
  cardinality: "one" | "maybeOne" | "many" | "none";
  cardinalityReason?: string;
  rewritten: { sql: string; order: string[]; spans: [number, number, number, number][] };
  style: "none" | "named" | "positional" | "mixed";
  diagnostics: Diagnostic[];
  unresolved: string[];
};

export type Lexed = {
  params: string[];
  placeholders: { name: string; start: number; end: number }[];
  style: "none" | "named" | "positional" | "mixed";
  /** The spans the lexer read as text (strings, quoted identifiers, comments): `[start, end]` byte offsets, in order. */
  opaque: [number, number][];
};

export type Need =
  | { kind: "describe"; id: string; sql: string }
  | { kind: "query"; id: string; sql: string; params: unknown[]; scope: "session" | "statement" };

export type Described = { params: { oid: number }[]; columns: { name: string; oid: number; table: number; attnum: number }[] };
/** MySQL's prepare metadata: the protocol type code, the flags (bit 1 is NOT NULL), the charset (63 is binary), the display length and the table the column is one of (empty for an expression). */
export type MysqlDescribed = { kind: "mysql"; columns: { name: string; type: number; flags: number; charset: number; length: number; org_table: string; org_name: string }[]; params: { type: number; flags: number; charset: number }[] };
export type DbError = { message: string; code?: string; position?: number };
export type Answer =
  | { describe: { params: number[]; columns: Described["columns"] } }
  | { mysql: { columns: MysqlDescribed["columns"]; params: MysqlDescribed["params"] } }
  | { rows: unknown[][] }
  | { error: DbError };

/** What the host does for the engine: describe a statement, run a catalog query. */
export interface EngineIo {
  /** Postgres: Parse/Describe. MySQL: a server prepare. SQLite needs none: the engine prepares inside its own SQLite. */
  describe(sql: string): Promise<Described | MysqlDescribed>;
  /** Rows as arrays of values, in column order. */
  query(sql: string, params: unknown[]): Promise<unknown[][]>;
}

type Response =
  | { status: "ok"; version: string; fingerprint: string }
  | { status: "lexed"; lexed: Lexed }
  | { status: "need"; needs: Need[] }
  | { status: "analysis"; analysis: Analysis }
  | { status: "config"; config: unknown }
  | { status: "configError"; error: { message: string; start: number; end: number } }
  | { status: "error"; message: string };

const WASM_PATH = new URL("../engine/reylo_engine.wasm", import.meta.url);

// The WebAssembly namespace is a global in every runtime reylo supports; it is typed here
// rather than through the DOM lib.
type Memory = { buffer: ArrayBuffer };
type Instance = { exports: Record<string, unknown> };
const WA = (globalThis as unknown as { WebAssembly: { compile(bytes: Uint8Array): Promise<unknown>; instantiate(module: unknown, imports: object): Promise<Instance> } }).WebAssembly;

// Node warns "ExperimentalWarning: WASI is an experimental feature" when `node:wasi` first loads
// in a thread, not when a WASI is made, so a static import printed that line before anything else
// in every program run under `reylo/register`. The module is loaded here instead, when the engine
// is, while a filter on `process.emitWarning`, which Node's own warning goes through, swallows that
// one warning and passes every other through. Node 24 and Bun both have `getBuiltinModule`.
const WASI_WARNING = "WASI is an experimental feature and might change at any time";
function loadWasi(): { WASI: typeof WasiClass } {
  const emitWarning = process.emitWarning;
  process.emitWarning = function (this: unknown, warning: string | Error, ...rest: unknown[]) {
    const message = typeof warning === "string" ? warning : warning.message;
    const type = typeof warning === "string" ? (typeof rest[0] === "string" ? rest[0] : (rest[0] as { type?: string } | undefined)?.type) : warning.name;
    if (message === WASI_WARNING && type === "ExperimentalWarning") return;
    return (emitWarning as (...args: unknown[]) => void).call(this, warning, ...rest);
  } as typeof process.emitWarning;
  try {
    return process.getBuiltinModule("node:wasi");
  } finally {
    process.emitWarning = emitWarning;
  }
}

export class Engine {
  #memory: Memory;
  #alloc: (len: number) => number;
  #free: (ptr: number, len: number) => void;
  #call: (ptr: number, len: number) => number;

  private constructor(instance: Instance) {
    const e = instance.exports as Record<string, any>;
    this.#memory = e.memory;
    this.#alloc = e.reylo_alloc;
    this.#free = e.reylo_free;
    this.#call = e.reylo_call;
  }

  static async load(path: URL | string = WASM_PATH): Promise<Engine> {
    const module = await WA.compile(readFileSync(path));
    const { WASI } = loadWasi();
    const wasi = new WASI({ version: "preview1", args: ["reylo-engine"], env: {} });
    // Bun's node:wasi has neither getImportObject nor initialize: it takes the imports from
    // wasiImport, and its start(instance) only sets the memory of a reactor, which has no _start.
    // Its random_get returns the length instead of 0 before Bun 1.4 (oven-sh/bun#35438), which
    // the engine's first hash map reads as a failure, so it is replaced.
    const bun = typeof (wasi as { getImportObject?: unknown }).getImportObject !== "function";
    let instance: Instance;
    const random_get = (ptr: number, len: number) => {
      const memory = instance.exports.memory as Memory;
      for (let at = 0; at < len; at += 65536) crypto.getRandomValues(new Uint8Array(memory.buffer, ptr + at, Math.min(65536, len - at)));
      return 0;
    };
    instance = await WA.instantiate(module, bun ? { wasi_snapshot_preview1: { ...wasi.wasiImport, random_get } } : wasi.getImportObject());
    if (bun) wasi.start(instance as never);
    else wasi.initialize(instance as never);
    return new Engine(instance);
  }

  call(request: object): Response {
    const bytes = new TextEncoder().encode(JSON.stringify(request));
    const ptr = this.#alloc(bytes.length);
    new Uint8Array(this.#memory.buffer, ptr, bytes.length).set(bytes);
    const out = this.#call(ptr, bytes.length);
    this.#free(ptr, bytes.length);
    const len = new DataView(this.#memory.buffer).getUint32(out, true);
    const text = new TextDecoder().decode(new Uint8Array(this.#memory.buffer, out + 4, len));
    this.#free(out, len + 4);
    return JSON.parse(text);
  }

  lex(sql: string, dialect: Dialect): Lexed {
    const r = this.call({ op: "lex", sql, dialect });
    if (r.status !== "lexed") throw new Error(r.status === "error" ? r.message : `unexpected ${r.status}`);
    return r.lexed;
  }

  /** Analyze one statement, fulfilling the engine's needs through `io` until it answers. */
  async analyze(sql: string, dialect: Dialect, io: EngineIo, cache = new AnswerCache()): Promise<Analysis> {
    const answers: Record<string, Answer> = {};
    for (let round = 0; round < 16; round++) {
      const r = this.call({ op: "analyze", sql, dialect, answers });
      if (r.status === "analysis") return r.analysis;
      if (r.status !== "need") throw new Error(r.status === "error" ? r.message : `unexpected ${r.status}`);
      await Promise.all(r.needs.map(async (need) => { answers[need.id] = await cache.get(need, () => fulfil(need, io)); }));
    }
    throw new Error("the analysis engine did not converge");
  }
}

/**
 * The database cannot be reached, as opposed to the server answering a statement with an error.
 * It ends the analysis at once instead of becoming an answer: asking again cannot succeed, and a
 * driver that backs off between attempts (postgres.js waits up to 20 s) would hold the transform
 * for minutes.
 */
export class Unreachable extends Error {
  constructor(cause: unknown) {
    super(reason(cause), { cause });
  }
}

/** A driver error as one line: a refused connection is an AggregateError with no message of its own. */
export function reason(e: unknown): string {
  const err = e as { message?: string; code?: string; errors?: { message?: string }[] } | undefined;
  return err?.message || err?.errors?.map((x) => x.message).filter(Boolean).join("; ") || err?.code || String(e);
}

async function fulfil(need: Need, io: EngineIo): Promise<Answer> {
  try {
    if (need.kind === "describe") {
      const d = await io.describe(need.sql);
      if ("kind" in d) return { mysql: { columns: d.columns, params: d.params } };
      return { describe: { params: d.params.map((p) => p.oid), columns: d.columns } };
    }
    return { rows: await io.query(need.sql, need.params) };
  } catch (e) {
    if (e instanceof Unreachable) throw e;
    const err = e as Error & { code?: string; position?: string | number };
    const error: DbError = { message: err.message };
    if (err.code) error.code = err.code;
    if (err.position !== undefined) error.position = Number(err.position);
    return { error };
  }
}

/**
 * Answers the host may reuse. A session answer is about the database as a whole (which
 * operators return sets, which built-ins are shadowed); a statement answer is about a
 * statement's relations and types. Both go stale when the schema changes, so both expire:
 * a migration shows up within seconds, and a transform of the same text a moment later
 * costs no round trip.
 */
export class AnswerCache {
  #entries = new Map<string, { at: number; answer: Answer }>();
  #ttl: { session: number; statement: number };
  constructor(ttl = { session: 60_000, statement: 5_000 }) {
    this.#ttl = ttl;
  }

  async get(need: Need, fetch: () => Promise<Answer>): Promise<Answer> {
    const ttl = need.kind === "query" && need.scope === "session" ? this.#ttl.session : this.#ttl.statement;
    const hit = this.#entries.get(need.id);
    if (hit && Date.now() - hit.at < ttl) return hit.answer;
    const answer = await fetch();
    // A failure is not knowledge: the next transform asks again.
    if (!("error" in answer)) this.#entries.set(need.id, { at: Date.now(), answer });
    return answer;
  }

  clear() {
    this.#entries.clear();
  }
}

let loaded: Promise<Engine> | undefined;
/** The process-wide engine instance. */
export function engine(): Promise<Engine> {
  return (loaded ??= Engine.load());
}
