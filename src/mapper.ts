// The content mapper process. First happy path: Postgres, live database, scalar
// column and parameter types from Parse/Describe through the project's own
// driver, NOT NULL from the catalog for plain columns, named placeholders,
// cardinality directives, and SQLSTATE errors mapped into the .sql file.
import { existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { createHash } from "node:crypto";
import { dirname, join, relative } from "node:path";
import { serve } from "./rpc.ts";
import { lexPlaceholders } from "./lex.ts";
import { engine, AnswerCache } from "./engine.ts";
import { ident, renderType } from "./render.ts";
import { openPostgresDescriber, type Describer } from "./describe.ts";
import { openSqliteDescriber, sqliteType, type SqliteDescriber } from "./sqlite-describe.ts";
import { mysqlNotNull, mysqlType, openMysqlDescriber, type MysqlDescriber } from "./mysql-describe.ts";

type Options = { dialect?: "postgres" | "sqlite" | "mysql"; source?: "database"; url?: string; urlEnv?: string };
type Project = {
  projectDir: string;
  options: Options;
  url: string | undefined;
  dialect: "postgres" | "sqlite" | "mysql";
  describer: Promise<Describer> | undefined;
  sqlite: Promise<SqliteDescriber> | undefined;
  mysql: Promise<MysqlDescriber> | undefined;
  answers: AnswerCache;
  cache: Record<string, CacheEntry>;
};
type CacheEntry = { hash: string; params: string[]; cardinality: string; dialect: string; text?: string; order?: string[] };
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

function directive(sql: string, name: string): string | undefined {
  return new RegExp(`^\\s*--\\s*@${name}\\s+(\\S+)`, "m").exec(sql)?.[1];
}

function diag(messageText: string, start: number, length: number, code = 1) {
  return { messageText, start, length, code };
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
    const dotenv = readDotenv(projectDir);
    const urlEnv = options.urlEnv ?? "DATABASE_URL";
    const url = options.url ?? process.env[urlEnv] ?? dotenv[urlEnv];
    const cachePath = join(projectDir, ".lando", "queries.json");
    const cache = existsSync(cachePath) ? JSON.parse(readFileSync(cachePath, "utf8")) : {};
    const dialect = options.dialect ?? (url && /^(file:|sqlite:)/.test(url) ? "sqlite" : url && /^(mysql|mariadb):/.test(url) ? "mysql" : "postgres");
    projects.set(p.projectHandle, { projectDir, options, url, dialect, describer: undefined, sqlite: undefined, mysql: undefined, answers: new AnswerCache(), cache });
    const configIdentity = createHash("sha1").update(JSON.stringify([options, url ?? null])).digest("hex");
    return { configIdentity, watchedFiles: [join(projectDir, ".env")], optionDiagnostics };
  },
  async transform(params) {
    const p = params as { fileName: string; content: string; projectHandle: string };
    const project = projects.get(p.projectHandle)!;
    const sql = p.content;
    const diagnostics: ReturnType<typeof diag>[] = [];
    const emit = (paramsType: string, rowType: string, cardinality: string, names: string[]) => ({
      extension: ".ts" as const,
      diagnostics,
      text: [
        `import type { Query } from "lando";`,
        `export type Params = ${paramsType};`,
        `export type Row = ${rowType};`,
        `export declare const sql: ${JSON.stringify(sql)};`,
        `export declare const params: readonly [${names.map((n) => JSON.stringify(n)).join(", ")}];`,
        `declare const query: Query<Params, Row, ${JSON.stringify(cardinality)}, ${JSON.stringify(project.dialect)}>;`,
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
    if (!["many", "one", "maybeOne"].includes(cardinality)) {
      const at = sql.indexOf(cardinality);
      diagnostics.push(diag(`Unknown @returns value "${cardinality}"; expected many, one or maybeOne.`, at, cardinality.length, 3));
    }
    if (!project.url) {
      diagnostics.push(diag(`No database URL: set ${project.options.urlEnv ?? "DATABASE_URL"} in the environment or .env, or "url" in the content mapper options.`, 0, 1, 4));
      return emit("unknown", "unknown", cardinality, names);
    }
    const annotated = new Map<string, string>();
    for (const m of sql.matchAll(/^\s*--\s*@param\s+(\S+)\s+(.+?)\s*$/gm)) annotated.set(m[1]!, m[2]!);
    const writeCache = (extra: Partial<CacheEntry> = {}) => {
      const key = relative(project.projectDir, p.fileName);
      project.cache[key] = { hash: createHash("sha1").update(sql).digest("hex"), params: names, cardinality, dialect: project.dialect, ...extra };
      mkdirSync(join(project.projectDir, ".lando"), { recursive: true });
      writeFileSync(join(project.projectDir, ".lando", "queries.json"), JSON.stringify(project.cache, null, 2) + "\n");
    };
    if (project.dialect === "mysql") {
      project.mysql ??= openMysqlDescriber(project.projectDir, project.url);
      let described;
      let mysqlText = "";
      try {
        // MySQL has only `?`; a repeated name repeats the placeholder.
        let text = "";
        let last = 0;
        for (const h of placeholders) { text += sql.slice(last, h.start) + "?"; last = h.end; }
        text += sql.slice(last);
        mysqlText = text;
        described = await (await project.mysql).describe(text);
      } catch (e) {
        const err = e as Error & { sqlState?: string; errno?: number };
        if (!err.sqlState) project.mysql = undefined;
        diagnostics.push(diag(`${err.message}${err.sqlState ? ` (${err.sqlState})` : ""}`, 0, Math.max(1, sql.trimEnd().length), 6));
        return emit("unknown", "unknown", cardinality, names);
      }
      const columns = described.columns.map((c) => `${ident(c.name)}: ${mysqlType(c)}${mysqlNotNull(c.flags) ? "" : " | null"}`);
      // MySQL 8.4's prepare infers a type per `?` from context (LONGLONG against an
      // int column, VAR_STRING against a varchar); an annotation overrides it, and a
      // diagnostic asks for one only when the server offers nothing usable.
      const paramTypes = names.map((name) => {
        const t = annotated.get(name);
        if (t) return `${ident(name)}: ${t}`;
        const index = placeholders.findIndex((h) => h.name === name);
        const guess = described.params[index];
        const inferred = guess ? mysqlType({ type: guess.type }) : "unknown";
        if (inferred === "unknown") {
          const at = placeholders[index]!;
          diagnostics.push(diag(`MySQL could not type parameter "${name}"; add "-- @param ${name} <type>".`, at.start, at.end - at.start, 7));
        }
        return `${ident(name)}: ${inferred}`;
      });
      writeCache({ text: mysqlText, order: placeholders.map((h) => h.name) });
      return emit(names.length ? `{ ${paramTypes.join("; ")} }` : "Record<never, never>", columns.length ? `{ ${columns.join("; ")} }` : "never", cardinality, names);
    }
    if (project.dialect === "sqlite") {
      // `-- @param name type` supplies what SQLite cannot: parameter types.
      project.sqlite ??= openSqliteDescriber(project.url, project.projectDir);
      let described;
      try {
        described = await (await project.sqlite).describe(sql);
      } catch (e) {
        project.sqlite = undefined;
        diagnostics.push(diag((e as Error).message, 0, Math.max(1, sql.trimEnd().length), 6));
        return emit("unknown", "unknown", cardinality, names);
      }
      const columns = described.columns.map((c) => {
        const t = c.table ? sqliteType(c.declared) : "unknown";
        return `${/^[A-Za-z_$][\w$]*$/.test(c.name) ? c.name : JSON.stringify(c.name)}: ${t}${described.notNull.has(c.name) ? "" : " | null"}`;
      });
      const paramTypes = names.map((name) => {
        const t = annotated.get(name);
        if (!t) {
          const at = placeholders.find((h) => h.name === name)!;
          diagnostics.push(diag(`SQLite cannot type parameter "${name}"; add "-- @param ${name} <type>" (number, string, Uint8Array, ...).`, at.start, at.end - at.start, 7));
        }
        return `${/^[A-Za-z_$][\w$]*$/.test(name) ? name : JSON.stringify(name)}: ${t ?? "unknown"}`;
      });
      writeCache();
      return emit(names.length ? `{ ${paramTypes.join("; ")} }` : "Record<never, never>", columns.length ? `{ ${columns.join("; ")} }` : "never", cardinality, names);
    }
    project.describer ??= openPostgresDescriber(project.projectDir, project.url);
    let describer: Describer;
    try {
      describer = await project.describer;
    } catch (e) {
      project.describer = undefined;
      diagnostics.push(diag(`Cannot connect to the database: ${(e as Error).message}`, 0, 1, 5));
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
      const err = e as Error & { code?: string };
      // A dead connection is reopened on the next transform; a server error is reported.
      if (!err.code) project.describer = undefined;
      diagnostics.push(diag(err.message, 0, 1, 5));
      return emit("unknown", "unknown", cardinality, names);
    }
    diagnostics.push(...analysis.diagnostics.map((d) => diag(d.message, d.start, d.length, d.code)));
    const inferred = directive(sql, "returns") ? cardinality : analysis.cardinality;
    if (analysis.diagnostics.some((d) => d.code === 6) || analysis.kind === "") {
      return emit("unknown", "unknown", inferred, analysis.rewritten.order);
    }
    const columns = analysis.columns.map((c) => `${ident(c.name)}: ${renderType(c.type)}${c.nullable ? " | null" : ""}`);
    const paramTypes = analysis.params.map((q) => `${ident(q.name)}: ${renderType(q.type)}`);
    writeCache({ cardinality: inferred, text: analysis.rewritten.sql, order: analysis.rewritten.order });
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
