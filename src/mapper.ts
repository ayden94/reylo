// The content mapper process. First happy path: Postgres, live database, scalar
// column and parameter types from Parse/Describe through the project's own
// driver, NOT NULL from the catalog for plain columns, named placeholders,
// cardinality directives, and SQLSTATE errors mapped into the .sql file.
import { existsSync, mkdirSync, readFileSync, renameSync, writeFileSync } from "node:fs";
import { createHash } from "node:crypto";
import { basename, dirname, join, relative } from "node:path";
import { serve } from "./rpc.ts";
import { lexPlaceholders } from "./lex.ts";
import { engine, reason, AnswerCache, Unreachable, type Analysis } from "./engine.ts";
import { conversion, defaultRepr, ident, renderParamType, renderType, type Conversion, type Repr } from "./render.ts";
import { openMysqlDescriber, type MysqlDescriber } from "./mysql-describe.ts";
import { openPostgresDescriber, type Describer } from "./describe.ts";
import { openSqliteDescriber, sqlitePath, type SqliteDescriber } from "./sqlite-describe.ts";

type Options = { dialect?: "postgres" | "sqlite" | "mysql"; source?: "database"; url?: string; urlEnv?: string; dates?: Repr["dates"]; bigint?: Repr["bigint"] };
type Project = {
  projectDir: string;
  options: Options;
  url: string | undefined;
  /** Where the URL came from, for a diagnostic: the option, the environment or `.env`. */
  urlSource: string;
  dialect: "postgres" | "sqlite" | "mysql";
  repr: Repr;
  describer: Promise<Describer> | undefined;
  sqlite: Promise<SqliteDescriber> | undefined;
  mysql: Promise<MysqlDescriber> | undefined;
  answers: AnswerCache;
  cache: Record<string, CacheEntry>;
};
type CacheEntry = { hash: string; params: string[]; cardinality: string; dialect: string; text?: string; order?: string[]; convert?: Record<string, Conversion>; dates?: Repr["dates"]; bigint?: Repr["bigint"] };
const projects = new Map<string, Project>();

function readDotenv(dir: string): Record<string, string> {
  const path = join(dir, ".env");
  if (!existsSync(path)) return {};
  const out: Record<string, string> = {};
  for (const line of readFileSync(path, "utf8").split("\n")) {
    const m = /^\s*(?:export\s+)?([A-Za-z_][A-Za-z0-9_]*)\s*=\s*(.*?)\s*$/.exec(line);
    if (m) out[m[1]!] = m[2]!.replace(/^(["'])(.*)\1$/, "$2");
  }
  return out;
}

/** The cache as it is on disk; a missing or half-written file is an empty cache. */
function readCache(projectDir: string): Record<string, CacheEntry> {
  const path = join(projectDir, ".lando", "queries.json");
  if (!existsSync(path)) return {};
  try {
    return JSON.parse(readFileSync(path, "utf8"));
  } catch {
    return {};
  }
}

function directive(sql: string, name: string): string | undefined {
  return new RegExp(`^\\s*--\\s*@${name}\\s+(\\S+)`, "m").exec(sql)?.[1];
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
  if (project.dialect === "sqlite") return diag(`Cannot open the database ${sqlitePath(project.url ?? "", project.projectDir)} (${project.urlSource}): ${reason(e)}`, 0, 1, 5);
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
    return { protocolVersion: protocolVersion ?? 1, positionEncoding: "utf-8", diagnosticSource: "lando" };
  },
  openProject(params) {
    const p = params as { projectHandle: string; options?: Options; configFileName: string };
    const projectDir = dirname(p.configFileName);
    const options = p.options ?? {};
    const optionDiagnostics: { path: (string | number)[]; messageText: string; code: number }[] = [];
    if (options.dialect !== undefined && !["postgres", "sqlite", "mysql"].includes(options.dialect)) optionDiagnostics.push({ path: ["dialect"], messageText: `Expected "postgres", "sqlite" or "mysql".`, code: 100 });
    if (options.source !== undefined && options.source !== "database") optionDiagnostics.push({ path: ["source"], messageText: `Only "database" is implemented so far.`, code: 101 });
    if (options.dates !== undefined && !["string", "Date", "temporal"].includes(options.dates)) optionDiagnostics.push({ path: ["dates"], messageText: `Expected "string" (ISO 8601 text, the default), "Date" or "temporal".`, code: 102 });
    if (options.bigint !== undefined && !["string", "number", "bigint"].includes(options.bigint)) optionDiagnostics.push({ path: ["bigint"], messageText: `Expected "string" (the default), "number" or "bigint".`, code: 103 });
    const repr: Repr = { dates: ["string", "Date", "temporal"].includes(options.dates as string) ? options.dates! : defaultRepr.dates, bigint: ["string", "number", "bigint"].includes(options.bigint as string) ? options.bigint! : defaultRepr.bigint };
    const dotenv = readDotenv(projectDir);
    const urlEnv = options.urlEnv ?? "DATABASE_URL";
    const url = options.url ?? process.env[urlEnv] ?? dotenv[urlEnv];
    const urlSource = options.url !== undefined ? `"url" in ${basename(p.configFileName)}` : process.env[urlEnv] !== undefined ? `${urlEnv} in the environment` : `${urlEnv} in .env`;
    const cache = readCache(projectDir);
    const dialect = options.dialect ?? (url && /^(file:|sqlite:)/.test(url) ? "sqlite" : url && /^(mysql|mariadb):/.test(url) ? "mysql" : "postgres");
    projects.set(p.projectHandle, { projectDir, options, url, urlSource, dialect, repr, describer: undefined, sqlite: undefined, mysql: undefined, answers: new AnswerCache(), cache });
    const configIdentity = createHash("sha1").update(JSON.stringify([options, url ?? null])).digest("hex");
    return { configIdentity, watchedFiles: [join(projectDir, ".env")], optionDiagnostics };
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
      return { extension: ".ts" as const, diagnostics, text: `import type { Query } from "lando";\nexport type Params = unknown;\nexport type Row = unknown;\ndeclare const query: Query<unknown, unknown, "many", "postgres">;\nexport default query;\n` };
    }
    const emit = (paramsType: string, rowType: string, cardinality: string, names: string[]) => ({
      extension: ".ts" as const,
      diagnostics,
      text: [
        `import type { Query } from "lando";`,
        // The Temporal types are not in TypeScript's libs yet; `lando/temporal` declares them.
        ...(project.repr.dates === "temporal" ? [`import "lando/temporal";`] : []),
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

    const { params: names, placeholders } = lexPlaceholders(sql);
    const named = names.filter((n) => !/^[$?]\d+$/.test(n));
    if (named.length && named.length !== names.length) {
      const first = placeholders.find((h) => /^[$?]\d+$/.test(h.name))!;
      diagnostics.push(diag("Named and positional parameters cannot be mixed in one statement.", first.start, first.end - first.start, 2));
      return emit("never", "never", "many", names);
    }
    const cardinality = directive(sql, "returns") ?? "many";
    if (!["many", "one", "maybeOne", "none"].includes(cardinality)) {
      const at = sql.indexOf(cardinality);
      diagnostics.push(diag(`Unknown @returns value "${cardinality}"; expected many, one, maybeOne or none.`, at, cardinality.length, 3));
    }
    if (!project.url) {
      diagnostics.push(diag(`No database URL: set ${project.options.urlEnv ?? "DATABASE_URL"} in the environment or .env, or "url" in the content mapper options.`, 0, 1, 4));
      return emit("unknown", "unknown", cardinality, names);
    }
    const writeCache = (extra: Partial<CacheEntry> = {}, convert: Record<string, Conversion> = {}) => {
      const key = relative(project.projectDir, p.fileName);
      // The runtime converts only the columns named here, to the representation named here.
      const conversions = Object.keys(convert).length ? { convert, dates: project.repr.dates, bigint: project.repr.bigint } : {};
      project.cache[key] = { hash: createHash("sha1").update(sql).digest("hex"), params: names, cardinality, dialect: project.dialect, ...extra, ...conversions };
      // Two editors on one project run two mappers; each writes the whole file, so the write
      // is a rename and a reader never sees a half-written cache.
      const dir = join(project.projectDir, ".lando");
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
      const inferred = directive(sql, "returns") ? cardinality : analysis.cardinality;
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
      project.sqlite ??= openSqliteDescriber(project.url, project.projectDir);
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
      const inferred = directive(sql, "returns") ? cardinality : analysis.cardinality;
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
    const inferred = directive(sql, "returns") ? cardinality : analysis.cardinality;
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
