// The analysis engine: `engine/lando_engine.wasm`, a WASI reactor, loaded once per process.
// The engine is pure. `analyze` replays it: every round it either answers or names what it
// still needs — statements to describe, catalog queries to run — and this module fulfils
// those through the project's own driver and calls again.
import { readFileSync } from "node:fs";
import { WASI } from "node:wasi";

export type Dialect = "postgres" | "sqlite" | "mysql";

export type EngineType =
  | { kind: "scalar"; name: string }
  | { kind: "enum"; name: string; labels: string[] }
  | { kind: "array"; of: EngineType }
  | { kind: "object"; fields: { name: string; type: EngineType; nullable: boolean }[] }
  | { kind: "list"; of: EngineType; nullable_items: boolean }
  | { kind: "json" }
  | { kind: "unknown" };

export type Diagnostic = { message: string; start: number; length: number; code: number };
export type Directive = { name: string; value: string; start: number; end: number };

export type Analysis = {
  kind: string;
  params: { name: string; type: EngineType; type_name: string }[];
  columns: { name: string; type: EngineType; nullable: boolean; type_name: string }[];
  cardinality: "one" | "maybeOne" | "many" | "none";
  cardinalityReason?: string;
  rewritten: { sql: string; order: string[]; spans: [number, number, number, number][] };
  style: "none" | "named" | "positional" | "mixed";
  directives: Directive[];
  diagnostics: Diagnostic[];
  unresolved: string[];
};

export type Lexed = {
  params: string[];
  placeholders: { name: string; start: number; end: number }[];
  directives: Directive[];
  style: "none" | "named" | "positional" | "mixed";
};

export type Need =
  | { kind: "describe"; id: string; sql: string }
  | { kind: "query"; id: string; sql: string; params: unknown[]; scope: "session" | "statement" };

export type Described = { params: { oid: number }[]; columns: { name: string; oid: number; table: number; attnum: number }[] };
/** MySQL's prepare metadata: the protocol type code, the flags (bit 1 is NOT NULL) and the charset (63 is binary). */
export type MysqlDescribed = { kind: "mysql"; columns: { name: string; type: number; flags: number; charset: number }[]; params: { type: number; flags: number; charset: number }[] };
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
  | { status: "error"; message: string };

const WASM_PATH = new URL("../engine/lando_engine.wasm", import.meta.url);

// The WebAssembly namespace is a global in every runtime Lando supports; it is typed here
// rather than through the DOM lib.
type Memory = { buffer: ArrayBuffer };
type Instance = { exports: Record<string, unknown> };
const WA = (globalThis as unknown as { WebAssembly: { compile(bytes: Uint8Array): Promise<unknown>; instantiate(module: unknown, imports: object): Promise<Instance> } }).WebAssembly;

export class Engine {
  #memory: Memory;
  #alloc: (len: number) => number;
  #free: (ptr: number, len: number) => void;
  #call: (ptr: number, len: number) => number;

  private constructor(instance: Instance) {
    const e = instance.exports as Record<string, any>;
    this.#memory = e.memory;
    this.#alloc = e.lando_alloc;
    this.#free = e.lando_free;
    this.#call = e.lando_call;
  }

  static async load(path: URL | string = WASM_PATH): Promise<Engine> {
    const module = await WA.compile(readFileSync(path));
    const wasi = new WASI({ version: "preview1", args: ["lando-engine"], env: {} });
    const instance = await WA.instantiate(module, wasi.getImportObject());
    wasi.initialize(instance as never);
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

async function fulfil(need: Need, io: EngineIo): Promise<Answer> {
  try {
    if (need.kind === "describe") {
      const d = await io.describe(need.sql);
      if ("kind" in d) return { mysql: { columns: d.columns, params: d.params } };
      return { describe: { params: d.params.map((p) => p.oid), columns: d.columns } };
    }
    return { rows: await io.query(need.sql, need.params) };
  } catch (e) {
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
