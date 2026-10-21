// The content mapper process: the type check's view of a `.sql` file. The analysis itself (the
// database, the engine, the `.reylo/` cache shared with the bundler plugins and `reylo generate`)
// is `src/analyze.ts`; this file renders it as types and places its diagnostics in the file.
import { createHash } from "node:crypto";
import { dirname } from "node:path";
import { serve } from "./rpc.ts";
import { anyEngine, type Analysis } from "./engine.ts";
import { declaredType, ident, renderParamType, renderType } from "./render.ts";
import { resolveProject, type Dialect } from "./project.ts";
import { Session } from "./analyze.ts";

type Project = {
  dialect: Dialect;
  strictParams: boolean;
  session: Promise<Session>;
};
const projects = new Map<string, Project>();

function diag(messageText: string, start: number, length: number, code = 1) {
  return { messageText, start, length, code };
}

/** A parameter's TypeScript type: `unknown` when nothing in the statement types it (a CAST does), with `| null` where the statement lets the caller pass null. */
function paramType(q: Analysis["params"][number], rendered: string): string {
  if (q.type.kind === "unknown") return "unknown";
  return q.nullable ? `${rendered} | null` : rendered;
}

/**
 * The span map TypeScript reads as `mappings`: `[virtualStart, virtualLength, originalStart,
 * originalLength, kind, features]`, in UTF-8 bytes, the encoding `initialize` chose. One atom takes the
 * whole module to the statement, so go-to-definition on anything the module declares (the import's
 * path, the query, a column, a parameter) opens the `.sql` file at its statement. TypeScript maps a
 * definition only when its whole range lies in one segment, and the import's path goes to the range
 * of the module from its first token to its end, which a map by name would cut into pieces. No hover,
 * references or rename, which would land on the whole statement or write the TypeScript into it.
 */
function mapToStatement(text: string, sql: string): number[][] {
  const bytes = Buffer.from(sql, "utf8");
  let start = 0;
  let end = bytes.length;
  while (start < end && /\s/.test(String.fromCharCode(bytes[start]!))) start++;
  while (end > start && /\s/.test(String.fromCharCode(bytes[end - 1]!))) end--;
  const ATOM = 1;
  const DEFINITION = 8, TYPE_DEFINITION = 16, IMPLEMENTATION = 32;
  return [[0, Buffer.byteLength(text), start, end - start, ATOM, DEFINITION | TYPE_DEFINITION | IMPLEMENTATION]];
}

serve({
  initialize(params) {
    // Builds from 2026-09-18 on send `protocolVersion: 1` and refuse a mapper that does not
    // answer with the version it speaks; the npm nightly of the same day does not send it yet.
    const { protocolVersion } = (params ?? {}) as { protocolVersion?: number };
    return { protocolVersion: protocolVersion ?? 1, positionEncoding: "utf-8", diagnosticSource: "reylo" };
  },
  async openProject(params) {
    const p = params as { projectHandle: string; options?: Record<string, unknown>; configFileName: string };
    const projectDir = dirname(p.configFileName);
    // Every setting lives in the project file, which PorgREST and Reylo Framework read too.
    const optionDiagnostics = Object.keys(p.options ?? {}).map((key) => ({ path: [key], messageText: `reylo reads its settings from .reylo.yaml, not from the content mapper's options; move "${key}" there.`, code: 104 }));
    const resolved = resolveProject(await anyEngine(projectDir), projectDir);
    projects.set(p.projectHandle, { dialect: resolved.dialect, strictParams: resolved.strictParams, session: Session.open(projectDir) });
    const configIdentity = createHash("sha1").update(JSON.stringify([resolved.file ?? null, resolved.dialect, resolved.strictParams, resolved.url ?? null, resolved.error ?? null, p.options ?? null])).digest("hex");
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
      const text = `import type { Query } from "reylo";\nexport type Params = unknown;\nexport type Row = unknown;\ndeclare const query: Query<unknown, unknown, "many", "postgres">;\nexport default query;\n`;
      return { extension: ".ts" as const, diagnostics, text, mappings: mapToStatement(text, sql) };
    }
    // `compiled` is the statement as Reylo sends it to the database (the engine's rewritten text),
    // which an analysis that failed has not given: then `sql` is any string.
    const emit = (compiled: string | undefined, paramsType: string, rowType: string, cardinality: string, names: string[]) => {
      const text = [
        `import type { Query } from "reylo";`,
        `export type Params = ${paramsType};`,
        `export type Row = ${rowType};`,
        `export declare const sql: ${compiled === undefined ? "string" : JSON.stringify(compiled)};`,
        `export declare const raw: ${JSON.stringify(sql)};`,
        `export declare const params: readonly [${names.map((n) => JSON.stringify(n)).join(", ")}];`,
        // The literal types go in the Query type directly rather than through the aliases, so a
        // hover shows the row's shape (`{ id: number; ... } | null`) and not `Row | null`. The two
        // texts go in by `typeof`, so a hover over the query names them and one over its `sql`
        // shows the statement.
        `declare const query: Query<${paramsType}, ${rowType}, ${JSON.stringify(cardinality)}, ${JSON.stringify(project.dialect)}> & { readonly sql: typeof sql; readonly raw: typeof raw };`,
        `export default query;`,
        ``,
      ].join("\n");
      return { extension: ".ts" as const, diagnostics, text, mappings: mapToStatement(text, sql) };
    };

    // The engine's lexer names the parameters. A statement it cannot read (a placeholder form the
    // dialect would misread, named and positional mixed) comes back from the analysis as an error
    // at its place; until then nothing is proved, so the row count is `many`.
    let names: string[] = [];
    let placeholders: { name: string; start: number; end: number }[] = [];
    const session = await project.session;
    try {
      ({ params: names, placeholders } = session.lex(sql));
    } catch {}
    const cardinality = "many";
    // A parameter nothing in the statement types is an error at its first use, unless the project
    // file says `strict_params: false` (the default is true since 2026-10-20).
    // A statement the engine already refused (a parameter written twice, an error from the server)
    // was not analysed to its end, so its parameters are untyped for that reason: its own error is
    // the one to fix first.
    const strictParams = (analysis: Analysis) => {
      if (!project.strictParams || analysis.diagnostics.length) return;
      for (const q of analysis.params) {
        if (q.type.kind !== "unknown") continue;
        const at = placeholders.find((h) => h.name === q.name);
        // The engine's offsets are UTF-8 bytes, as the diagnostics' are. A MySQL placeholder's span
        // takes in the `::type` the engine rewrote, which is no part of its name.
        const span = at ? Buffer.from(sql, "utf8").subarray(at.start, at.end).toString("utf8") : `:${q.name}`;
        const written = span.replace(/\s*::[\s\S]*$/, "");
        const length = at ? Buffer.byteLength(written) : 1;
        const message = `Nothing in the statement gives the parameter ${written} a type; cast it, as in ${written}::text, or set strict_params: false in .reylo.yaml.`;
        diagnostics.push(diag(message, at?.start ?? 0, length, 12));
      }
    };
    const outcome = await session.analyze(sql);
    if (!outcome.ok) {
      diagnostics.push(diag(outcome.message, 0, 1, outcome.code));
      return emit(undefined, "unknown", "unknown", cardinality, names);
    }
    const { analysis } = outcome;
    const dialect = project.dialect;
    // A MySQL session reads its strings by its sql_mode, which the analysis asked for and the
    // lexer alone does not know: the placeholders are the ones the analysis read.
    if (dialect === "mysql") {
      names = analysis.params.map((q) => q.name);
      placeholders = analysis.rewritten.spans.map(([start, end], k) => ({ name: analysis.rewritten.order[k]!, start, end }));
    }
    diagnostics.push(...analysis.diagnostics.map((d) => diag(d.message, d.start, d.length, d.code)));
    strictParams(analysis);
    const inferred = analysis.cardinality;
    // The declared names are the lexer's, in order of first use: the module's `params` at run time.
    // Each parameter appears once in a statement; a second use is the engine's diagnostic (code 3),
    // placed above.
    const order = names;
    if (analysis.diagnostics.some((d) => d.code === 6) || (dialect === "postgres" && analysis.kind === "")) return emit(analysis.rewritten.sql, "unknown", "unknown", inferred, order);
    const columns = analysis.columns.map((c) => `${ident(c.name)}: ${renderType(c.type, false, dialect)}${c.nullable ? " | null" : ""}`);
    // A parameter is typed from its context (a column it is compared with or written to, a CAST);
    // one no context types is `unknown`.
    const typed = analysis.params.map((q) => [q.name, paramType(q, declaredType(q.type, q.schema) ?? renderParamType(q.type, dialect))] as const);
    // A typed JSON parameter whose schema has a `default` may be left out (wiki/typed-json-params.md).
    const optional = new Set(analysis.params.filter((q) => q.schema !== null && typeof q.schema === "object" && "default" in q.schema).map((q) => q.name));
    const paramTypes = typed.map(([name, type]) => `${ident(name)}${optional.has(name) ? "?" : ""}: ${type}`);
    return emit(analysis.rewritten.sql, paramTypes.length ? `{ ${paramTypes.join("; ")} }` : "Record<never, never>", columns.length ? `{ ${columns.join("; ")} }` : "never", inferred, order);
  },
  async closeProject(params) {
    const handle = (params as { projectHandle: string }).projectHandle;
    const project = projects.get(handle);
    projects.delete(handle);
    if (project) await (await project.session).close();
    return {};
  },
});
