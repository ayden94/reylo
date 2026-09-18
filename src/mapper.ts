// The content mapper process. First happy path: Postgres, live database, scalar
// column and parameter types from Parse/Describe through the project's own
// driver, NOT NULL from the catalog for plain columns, named placeholders,
// cardinality directives, and SQLSTATE errors mapped into the .sql file.
import { existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { createHash } from "node:crypto";
import { dirname, join, relative } from "node:path";
import { serve } from "./rpc.ts";
import { lexPlaceholders, type Placeholder } from "./lex.ts";
import { openPostgresDescriber, type Describer } from "./describe.ts";
import { PG_TYPE_QUERY, typeText, type PgTypeRow } from "./pgtypes.ts";

type Options = { dialect?: "postgres"; source?: "database"; url?: string; urlEnv?: string };
type Project = {
  projectDir: string;
  options: Options;
  url: string | undefined;
  describer: Promise<Describer> | undefined;
  cache: Record<string, { hash: string; params: string[]; cardinality: string }>;
};
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

function rewriteToDollar(sql: string, placeholders: Placeholder[], params: string[]): { text: string; toOriginal: (pos: number) => number } {
  let text = "";
  let last = 0;
  const shifts: { at: number; delta: number }[] = [];
  for (const p of placeholders) {
    const replacement = `$${params.indexOf(p.name) + 1}`;
    text += sql.slice(last, p.start) + replacement;
    shifts.push({ at: text.length, delta: p.end - p.start - replacement.length });
    last = p.end;
  }
  text += sql.slice(last);
  const toOriginal = (pos: number) => shifts.reduce((acc, s) => (pos > s.at ? acc + s.delta : acc), pos);
  return { text, toOriginal };
}

function directive(sql: string, name: string): string | undefined {
  return new RegExp(`^\\s*--\\s*@${name}\\s+(\\S+)`, "m").exec(sql)?.[1];
}

function diag(messageText: string, start: number, length: number, code = 1) {
  return { messageText, start, length, code };
}

serve({
  initialize() {
    return { positionEncoding: "utf-8", diagnosticSource: "lando" };
  },
  openProject(params) {
    const p = params as { projectHandle: string; options?: Options; configFileName: string };
    const projectDir = dirname(p.configFileName);
    const options = p.options ?? {};
    const optionDiagnostics: { path: (string | number)[]; messageText: string; code: number }[] = [];
    if (options.dialect !== undefined && options.dialect !== "postgres") optionDiagnostics.push({ path: ["dialect"], messageText: `Only "postgres" is implemented so far.`, code: 100 });
    if (options.source !== undefined && options.source !== "database") optionDiagnostics.push({ path: ["source"], messageText: `Only "database" is implemented so far.`, code: 101 });
    const dotenv = readDotenv(projectDir);
    const urlEnv = options.urlEnv ?? "DATABASE_URL";
    const url = options.url ?? process.env[urlEnv] ?? dotenv[urlEnv];
    const cachePath = join(projectDir, ".lando", "queries.json");
    const cache = existsSync(cachePath) ? JSON.parse(readFileSync(cachePath, "utf8")) : {};
    projects.set(p.projectHandle, { projectDir, options, url, describer: undefined, cache });
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
        `declare const query: Query<Params, Row, ${JSON.stringify(cardinality)}, "postgres">;`,
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
    project.describer ??= openPostgresDescriber(project.projectDir, project.url);
    let describer: Describer;
    try {
      describer = await project.describer;
    } catch (e) {
      project.describer = undefined;
      diagnostics.push(diag(`Cannot connect to the database: ${(e as Error).message}`, 0, 1, 5));
      return emit("unknown", "unknown", cardinality, names);
    }
    const { text, toOriginal } = rewriteToDollar(sql, placeholders, names);
    let described;
    try {
      described = await describer.describe(text);
    } catch (e) {
      const err = e as Error & { code?: string; position?: string | number };
      const pos = err.position ? toOriginal(Number(err.position) - 1) : 0;
      diagnostics.push(diag(`${err.message}${err.code ? ` (${err.code})` : ""}`, pos, Math.max(1, /^[\w."]+/.exec(sql.slice(pos))?.[0].length ?? 1), 6));
      return emit("unknown", "unknown", cardinality, names);
    }
    const lookup = async (oids: number[]) => (await describer.query(PG_TYPE_QUERY, [oids])) as PgTypeRow[];
    const tableOids = [...new Set(described.columns.map((c) => c.table).filter((t) => t > 0))];
    const notNull = new Set<string>();
    if (tableOids.length) {
      const rows = (await describer.query(`select attrelid::int as attrelid, attnum::int as attnum from pg_attribute where attrelid = any($1::oid[]) and attnotnull`, [tableOids])) as { attrelid: number; attnum: number }[];
      for (const r of rows) notNull.add(`${r.attrelid}:${r.attnum}`);
    }
    // Until the engine's join analysis lands, a plain column is non-null only when the
    // statement has no join, set operation or outer reference that could null it.
    const simpleShape = !/\b(join|union|except|intersect)\b/i.test(sql);
    const columns: string[] = [];
    for (const c of described.columns) {
      const t = await typeText(c.oid, lookup);
      const nonNull = simpleShape && c.table > 0 && notNull.has(`${c.table}:${c.attnum}`);
      columns.push(`${/^[A-Za-z_$][\w$]*$/.test(c.name) ? c.name : JSON.stringify(c.name)}: ${t}${nonNull ? "" : " | null"}`);
    }
    const paramTypes: string[] = [];
    for (const [i, name] of names.entries()) {
      const t = await typeText(described.params[i]?.oid ?? 0, lookup);
      paramTypes.push(`${/^[A-Za-z_$][\w$]*$/.test(name) ? name : JSON.stringify(name)}: ${t}`);
    }
    const paramsType = named.length ? `{ ${paramTypes.join("; ")} }` : names.length ? `{ ${paramTypes.join("; ")} }` : "Record<never, never>";
    const rowType = columns.length ? `{ ${columns.join("; ")} }` : "never";
    const key = relative(project.projectDir, p.fileName);
    project.cache[key] = { hash: createHash("sha1").update(sql).digest("hex"), params: names, cardinality };
    mkdirSync(join(project.projectDir, ".lando"), { recursive: true });
    writeFileSync(join(project.projectDir, ".lando", "queries.json"), JSON.stringify(project.cache, null, 2) + "\n");
    return emit(paramsType, rowType, cardinality, names);
  },
  async closeProject(params) {
    const project = projects.get((params as { projectHandle: string }).projectHandle);
    projects.delete((params as { projectHandle: string }).projectHandle);
    if (project?.describer) await (await project.describer).close().catch(() => {});
    return {};
  },
});
