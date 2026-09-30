// The content mapper process. First happy path: Postgres, live database, scalar
// column and parameter types from Parse/Describe through the project's own
// driver, NOT NULL from the catalog for plain columns, named placeholders,
// the proved row count, and SQLSTATE errors mapped into the .sql file.
import { existsSync, mkdirSync, readFileSync, renameSync, writeFileSync } from "node:fs";
import { createHash } from "node:crypto";
import { dirname, join, relative } from "node:path";
import { serve } from "./rpc.ts";
import { engine, reason, AnswerCache, Unreachable, type Analysis } from "./engine.ts";
import { conversion, ident, renderParamType, renderType, type Conversion, type Repr } from "./render.ts";
import { resolveProject, type Dialect } from "./project.ts";
import { openMysqlDescriber, type MysqlDescriber } from "./mysql-describe.ts";
import { openPostgresDescriber, type Describer } from "./describe.ts";
import { openSqliteDescriber, sqlitePath, type SqliteDescriber } from "./sqlite-describe.ts";

type Project = {
  projectDir: string;
  /** The directory a relative SQLite path resolves against: the project file's, or the tsconfig's. */
  root: string;
  url: string | undefined;
  /** Where the URL came from, for a diagnostic: the project file, the environment or a `.env` file. */
  urlSource: string;
  dialect: Dialect;
  repr: Repr;
  /** Why the project file cannot be used; every transform reports it. */
  error: string | undefined;
  paramTyping: "loose" | "strict";
  describer: Promise<Describer> | undefined;
  sqlite: Promise<SqliteDescriber> | undefined;
  mysql: Promise<MysqlDescriber> | undefined;
  answers: AnswerCache;
  cache: Record<string, CacheEntry>;
};
type CacheEntry = { hash: string; params: string[]; cardinality: string; dialect: string; text?: string; order?: string[]; convert?: Record<string, Conversion>; dates?: Repr["dates"]; bigint?: Repr["bigint"] };
const projects = new Map<string, Project>();

/** The cache as it is on disk; a missing or half-written file is an empty cache. */
function readCache(projectDir: string): Record<string, CacheEntry> {
  const path = join(projectDir, ".porg", "queries.json");
  if (!existsSync(path)) return {};
  try {
    return JSON.parse(readFileSync(path, "utf8"));
  } catch {
    return {};
  }
}

function diag(messageText: string, start: number, length: number, code = 1) {
  return { messageText, start, length, code };
}

/** A parameter's TypeScript type: `unknown` when nothing in the statement types it (a CAST does), with `| null` where the statement lets the caller pass null. */
function paramType(q: Analysis["params"][number], rendered: string): string {
  if (q.type.kind === "unknown") return "unknown";
  return q.nullable ? `${rendered} | null` : rendered;
}

/** The diagnostic for a database that cannot be reached: where it was looked for, where that came from, and the driver's reason; never the credentials. */
function cannotConnect(project: Project, e: unknown) {
  if (project.dialect === "sqlite") return diag(`Cannot open the database ${sqlitePath(project.url ?? "", project.root)} (${project.urlSource}): ${reason(e)}`, 0, 1, 5);
  let where = "the database";
  try {
    const u = new URL(project.url ?? "");
    where = `${u.hostname || u.searchParams.get("host") || "localhost"}${u.port ? `:${u.port}` : ""}${u.pathname}`;
  } catch {}
  return diag(`Cannot connect to the database at ${where} (${project.urlSource}): ${reason(e)}`, 0, 1, 5);
}

serve({
  initialize(params) {
    // Builds from 2026-09-18 on send `protocolVersion: 1` and refuse a mapper that does not
    // answer with the version it speaks; the npm nightly of the same day does not send it yet.
    const { protocolVersion } = (params ?? {}) as { protocolVersion?: number };
    return { protocolVersion: protocolVersion ?? 1, positionEncoding: "utf-8", diagnosticSource: "porg" };
  },
  async openProject(params) {
    const p = params as { projectHandle: string; options?: Record<string, unknown>; configFileName: string };
    const projectDir = dirname(p.configFileName);
    // Every setting lives in the project file, which PorgREST and PorgAPI read too.
    const optionDiagnostics = Object.keys(p.options ?? {}).map((key) => ({ path: [key], messageText: `Porg reads its settings from .porg.toml, not from the content mapper's options; move "${key}" there.`, code: 104 }));
    const resolved = resolveProject(await engine(), projectDir);
    const cache = readCache(projectDir);
    projects.set(p.projectHandle, { projectDir, root: resolved.root, url: resolved.url, urlSource: resolved.urlSource, dialect: resolved.dialect, repr: resolved.repr, error: resolved.error, paramTyping: resolved.paramTyping, describer: undefined, sqlite: undefined, mysql: undefined, answers: new AnswerCache(), cache });
    const configIdentity = createHash("sha1").update(JSON.stringify([resolved.file ?? null, resolved.dialect, resolved.repr, resolved.paramTyping, resolved.url ?? null, resolved.error ?? null, p.options ?? null])).digest("hex");
    return { configIdentity, watchedFiles: resolved.watched, optionDiagnostics };
  },
  async transform(params) {
    const p = params as { fileName: string; content: string; projectHandle: string };
    const sql = p.content;
    const diagnostics: ReturnType<typeof diag>[] = [];
    const project = projects.get(p.projectHandle);
    if (!project) {
      // A transform for a project this process never opened: answer, never throw, so the file
      // keeps its default export instead of becoming an empty module.
      diagnostics.push(diag(`The content mapper has no project for this file; restart the TypeScript server.`, 0, 1, 9));
      return { extension: ".ts" as const, diagnostics, text: `import type { Query } from "porg";\nexport type Params = unknown;\nexport type Row = unknown;\ndeclare const query: Query<unknown, unknown, "many", "postgres">;\nexport default query;\n` };
    }
    const emit = (paramsType: string, rowType: string, cardinality: string, names: string[]) => ({
      extension: ".ts" as const,
      diagnostics,
      text: [
        `import type { Query } from "porg";`,
        // The Temporal types are not in TypeScript's libs yet; `porg/temporal` declares them.
        ...(project.repr.dates === "temporal" ? [`import "porg/temporal";`] : []),
        `export type Params = ${paramsType};`,
        `export type Row = ${rowType};`,
        `export declare const sql: ${JSON.stringify(sql)};`,
        `export declare const params: readonly [${names.map((n) => JSON.stringify(n)).join(", ")}];`,
        // The literal types go in the Query type directly rather than through the aliases, so a
        // hover shows the row's shape (`{ id: number; ... } | null`) and not `Row | null`.
        `declare const query: Query<${paramsType}, ${rowType}, ${JSON.stringify(cardinality)}, ${JSON.stringify(project.dialect)}>;`,
        `export default query;`,
        ``,
      ].join("\n"),
    });

    // The engine's lexer names the parameters. A statement it cannot read (a placeholder form the
    // dialect would misread, named and positional mixed) comes back from the analysis as an error
    // at its place; until then nothing is proved, so the row count is `many`.
    let names: string[] = [];
    let placeholders: { name: string; start: number; end: number }[] = [];
    try {
      ({ params: names, placeholders } = (await engine()).lex(sql, project.dialect));
    } catch {}
    const cardinality = "many";
    // `param_typing = "strict"`: a parameter nothing in the statement types is an error at its first use.
    const strict = (analysis: Analysis) => {
      if (project.paramTyping !== "strict") return;
      for (const q of analysis.params) {
        if (q.type.kind !== "unknown") continue;
        const at = placeholders.find((h) => h.name === q.name);
        // The engine's offsets are UTF-8 bytes, as the diagnostics' are.
        const written = at ? Buffer.from(sql, "utf8").subarray(at.start, at.end).toString("utf8") : `:${q.name}`;
        diagnostics.push(diag(`Nothing in the statement gives the parameter ${written} a type; cast it, as in cast(${written} as integer), or set param_typing = "loose" in .porg.toml.`, at?.start ?? 0, at ? at.end - at.start : 1, 12));
      }
    };
    if (project.error) {
      diagnostics.push(diag(project.error, 0, 1, 11));
      return emit("unknown", "unknown", cardinality, names);
    }
    if (!project.url) {
      diagnostics.push(diag(`No database URL: set DATABASE_URL in the environment, .env.local or .env, or "url" in .porg.toml.`, 0, 1, 4));
      return emit("unknown", "unknown", cardinality, names);
    }
    const writeCache = (extra: Partial<CacheEntry> = {}, convert: Record<string, Conversion> = {}) => {
      const key = relative(project.projectDir, p.fileName);
      // The runtime converts only the columns named here, to the representation named here.
      const conversions = Object.keys(convert).length ? { convert, dates: project.repr.dates, bigint: project.repr.bigint } : {};
      project.cache[key] = { hash: createHash("sha1").update(sql).digest("hex"), params: names, cardinality, dialect: project.dialect, ...extra, ...conversions };
      // Two editors on one project run two mappers; each writes the whole file, so the write
      // is a rename and a reader never sees a half-written cache.
      const dir = join(project.projectDir, ".porg");
      mkdirSync(dir, { recursive: true });
      const tmp = join(dir, `queries.json.${process.pid}.tmp`);
      writeFileSync(tmp, JSON.stringify(project.cache, null, 2) + "\n");
      renameSync(tmp, join(dir, "queries.json"));
    };
    if (project.dialect === "mysql") {
      project.mysql ??= openMysqlDescriber(project.projectDir, project.url);
      let mysql: MysqlDescriber;
      try {
        mysql = await project.mysql;
      } catch (e) {
        project.mysql = undefined;
        diagnostics.push(e instanceof Unreachable ? cannotConnect(project, e) : diag((e as Error).message, 0, 1, 5));
        return emit("unknown", "unknown", cardinality, names);
      }
      // The server's prepare is the type source (each column's type and NOT NULL, a type per
      // `?` from its context); the engine reads it and proves the row count over its own parse
      // and the tables' unique keys.
      let analysis;
      try {
        analysis = await (await engine()).analyze(sql, "mysql", {
          describe: (text) => mysql.describe(text),
          query: (text, params) => mysql.values(text, params),
        }, project.answers);
      } catch (e) {
        // A dead connection is reopened on the next transform.
        if (e instanceof Unreachable) project.mysql = undefined;
        diagnostics.push(e instanceof Unreachable ? cannotConnect(project, e) : diag((e as Error).message, 0, 1, 5));
        return emit("unknown", "unknown", cardinality, names);
      }
      diagnostics.push(...analysis.diagnostics.map((d) => diag(d.message, d.start, d.length, d.code)));
      strict(analysis);
      const inferred = analysis.cardinality;
      if (analysis.diagnostics.some((d) => d.code === 6)) return emit("unknown", "unknown", inferred, names);
      const columns = analysis.columns.map((c) => `${ident(c.name)}: ${renderType(c.type, false, "mysql", project.repr)}${c.nullable ? " | null" : ""}`);
      // The server types a `?` from its context (LONGLONG against an int column, VAR_STRING
      // against a varchar, the target of a CAST); one it leaves untyped is `unknown`.
      const paramTypes = analysis.params.map((q) => `${ident(q.name)}: ${paramType(q, renderParamType(q.type, "mysql", project.repr))}`);
      const convert: Record<string, Conversion> = {};
      for (const c of analysis.columns) { const cls = conversion(c.type); if (cls) convert[c.name] = cls; }
      writeCache({ cardinality: inferred, text: analysis.rewritten.sql, order: analysis.rewritten.order }, convert);
      return emit(names.length ? `{ ${paramTypes.join("; ")} }` : "Record<never, never>", columns.length ? `{ ${columns.join("; ")} }` : "never", inferred, names);
    }
    if (project.dialect === "sqlite") {
      project.sqlite ??= openSqliteDescriber(project.url, project.root);
      let sqlite: SqliteDescriber;
      try {
        sqlite = await project.sqlite;
      } catch (e) {
        project.sqlite = undefined;
        diagnostics.push(cannotConnect(project, e));
        return emit("unknown", "unknown", cardinality, names);
      }
      let analysis;
      try {
        // The engine prepares the statement inside its own SQLite against the schema it reads
        // through `values`; it never asks this side to describe.
        analysis = await (await engine()).analyze(sql, "sqlite", { describe: () => Promise.reject(new Error("SQLite is described inside the engine")), query: (text, params) => sqlite.values(text, params) }, project.answers);
      } catch (e) {
        project.sqlite = undefined;
        diagnostics.push(diag((e as Error).message, 0, 1, 5));
        return emit("unknown", "unknown", cardinality, names);
      }
      diagnostics.push(...analysis.diagnostics.map((d) => diag(d.message, d.start, d.length, d.code)));
      strict(analysis);
      const inferred = analysis.cardinality;
      if (analysis.diagnostics.some((d) => d.code === 6)) return emit("unknown", "unknown", inferred, analysis.rewritten.order);
      const columns = analysis.columns.map((c) => `${ident(c.name)}: ${renderType(c.type, false, "sqlite")}${c.nullable ? " | null" : ""}`);
      // The engine types a parameter from its context (a column it is compared with or written
      // to, a CAST); one no context types is `unknown`.
      const paramTypes = analysis.params.map((q) => `${ident(q.name)}: ${paramType(q, renderType(q.type, false, "sqlite"))}`);
      writeCache({ cardinality: inferred, text: analysis.rewritten.sql, order: analysis.rewritten.order });
      return emit(names.length ? `{ ${paramTypes.join("; ")} }` : "Record<never, never>", columns.length ? `{ ${columns.join("; ")} }` : "never", inferred, names);
    }
    project.describer ??= openPostgresDescriber(project.projectDir, project.url);
    let describer: Describer;
    try {
      describer = await project.describer;
    } catch (e) {
      project.describer = undefined;
      diagnostics.push(e instanceof Unreachable ? cannotConnect(project, e) : diag((e as Error).message, 0, 1, 5));
      return emit("unknown", "unknown", cardinality, names);
    }
    // The engine owns the analysis: it names what it needs (a describe, a catalog query),
    // the project's driver answers, and the engine settles types, nullability, nested JSON
    // shapes and the row count.
    let analysis;
    try {
      analysis = await (await engine()).analyze(sql, "postgres", {
        describe: (text) => describer.describe(text),
        query: (text, params) => describer.values(text, params),
      }, project.answers);
    } catch (e) {
      // A dead connection is reopened on the next transform; a server error is reported.
      if (e instanceof Unreachable) project.describer = undefined;
      diagnostics.push(e instanceof Unreachable ? cannotConnect(project, e) : diag((e as Error).message, 0, 1, 5));
      return emit("unknown", "unknown", cardinality, names);
    }
    diagnostics.push(...analysis.diagnostics.map((d) => diag(d.message, d.start, d.length, d.code)));
    strict(analysis);
    const inferred = analysis.cardinality;
    if (analysis.diagnostics.some((d) => d.code === 6) || analysis.kind === "") {
      return emit("unknown", "unknown", inferred, analysis.rewritten.order);
    }
    const columns = analysis.columns.map((c) => `${ident(c.name)}: ${renderType(c.type, false, "postgres", project.repr)}${c.nullable ? " | null" : ""}`);
    const paramTypes = analysis.params.map((q) => `${ident(q.name)}: ${paramType(q, renderParamType(q.type, "postgres", project.repr))}`);
    const convert: Record<string, Conversion> = {};
    for (const c of analysis.columns) { const cls = conversion(c.type); if (cls) convert[c.name] = cls; }
    writeCache({ cardinality: inferred, text: analysis.rewritten.sql, order: analysis.rewritten.order }, convert);
    return emit(
      paramTypes.length ? `{ ${paramTypes.join("; ")} }` : "Record<never, never>",
      columns.length ? `{ ${columns.join("; ")} }` : "never",
      inferred,
      analysis.rewritten.order,
    );
  },
  async closeProject(params) {
    const project = projects.get((params as { projectHandle: string }).projectHandle);
    projects.delete((params as { projectHandle: string }).projectHandle);
    if (project?.describer) await (await project.describer).close().catch(() => {});
    if (project?.sqlite) await (await project.sqlite).close().catch(() => {});
    if (project?.mysql) await (await project.mysql).close().catch(() => {});
    return {};
  },
});
