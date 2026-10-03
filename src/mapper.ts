// The content mapper process: the type check's view of a `.sql` file. The analysis itself (the
// database, the engine, the `.dotsql/` cache shared with the bundler plugins and `dotsql generate`)
// is `src/analyze.ts`; this file renders it as types and places its diagnostics in the file.
import { createHash } from "node:crypto";
import { dirname } from "node:path";
import { serve } from "./rpc.ts";
import { engine, type Analysis } from "./engine.ts";
import { ident, renderParamType, renderType, type Repr } from "./render.ts";
import { resolveProject, type Dialect } from "./project.ts";
import { Session } from "./analyze.ts";

type Project = {
  dialect: Dialect;
  repr: Repr;
  strict: boolean;
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

serve({
  initialize(params) {
    // Builds from 2026-09-18 on send `protocolVersion: 1` and refuse a mapper that does not
    // answer with the version it speaks; the npm nightly of the same day does not send it yet.
    const { protocolVersion } = (params ?? {}) as { protocolVersion?: number };
    return { protocolVersion: protocolVersion ?? 1, positionEncoding: "utf-8", diagnosticSource: "dotsql" };
  },
  async openProject(params) {
    const p = params as { projectHandle: string; options?: Record<string, unknown>; configFileName: string };
    const projectDir = dirname(p.configFileName);
    // Every setting lives in the project file, which PorgREST and PorgAPI read too.
    const optionDiagnostics = Object.keys(p.options ?? {}).map((key) => ({ path: [key], messageText: `dotsql reads its settings from .dotsql.yaml, not from the content mapper's options; move "${key}" there.`, code: 104 }));
    const resolved = resolveProject(await engine(), projectDir);
    projects.set(p.projectHandle, { dialect: resolved.dialect, repr: resolved.repr, strict: resolved.strict, session: Session.open(projectDir) });
    const configIdentity = createHash("sha1").update(JSON.stringify([resolved.file ?? null, resolved.dialect, resolved.repr, resolved.strict, resolved.url ?? null, resolved.error ?? null, p.options ?? null])).digest("hex");
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
      return { extension: ".ts" as const, diagnostics, text: `import type { Query } from "dotsql";\nexport type Params = unknown;\nexport type Values = unknown[];\nexport type Row = unknown;\ndeclare const query: Query<unknown, unknown, "many", "postgres">;\nexport default query;\n` };
    }
    const emit = (paramsType: string, rowType: string, cardinality: string, names: string[], valuesType = "unknown[]") => ({
      extension: ".ts" as const,
      diagnostics,
      text: [
        // TypeScript's own lib declares the Temporal types; the reference brings them in when
        // the project's `lib` stops short of `esnext`.
        ...(project.repr.dates === "temporal" ? [`/// <reference lib="esnext.temporal" />`] : []),
        `import type { Query } from "dotsql";`,
        `export type Params = ${paramsType};`,
        // The same parameters as an array, in the order of `params`, for `run(client, [1, "ada"])`.
        `export type Values = ${valuesType};`,
        `export type Row = ${rowType};`,
        `export declare const sql: ${JSON.stringify(sql)};`,
        `export declare const params: readonly [${names.map((n) => JSON.stringify(n)).join(", ")}];`,
        // The literal types go in the Query type directly rather than through the aliases, so a
        // hover shows the row's shape (`{ id: number; ... } | null`) and not `Row | null`.
        `declare const query: Query<${paramsType}, ${rowType}, ${JSON.stringify(cardinality)}, ${JSON.stringify(project.dialect)}, ${valuesType}>;`,
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
    // `strict: true`: a parameter nothing in the statement types is an error at its first use.
    const strict = (analysis: Analysis) => {
      if (!project.strict) return;
      for (const q of analysis.params) {
        if (q.type.kind !== "unknown") continue;
        const at = placeholders.find((h) => h.name === q.name);
        // The engine's offsets are UTF-8 bytes, as the diagnostics' are.
        const written = at ? Buffer.from(sql, "utf8").subarray(at.start, at.end).toString("utf8") : `:${q.name}`;
        diagnostics.push(diag(`Nothing in the statement gives the parameter ${written} a type; cast it, as in cast(${written} as integer), or remove strict: true from .dotsql.yaml.`, at?.start ?? 0, at ? at.end - at.start : 1, 12));
      }
    };
    const session = await project.session;
    const outcome = await session.analyze(sql);
    if (!outcome.ok) {
      diagnostics.push(diag(outcome.message, 0, 1, outcome.code));
      return emit("unknown", "unknown", cardinality, names);
    }
    const { analysis } = outcome;
    const dialect = project.dialect;
    diagnostics.push(...analysis.diagnostics.map((d) => diag(d.message, d.start, d.length, d.code)));
    strict(analysis);
    const inferred = analysis.cardinality;
    // The declared names are the lexer's, each once in order of first use: the module's `params`
    // at run time, which an array of values follows. (A rewritten statement can bind one name at
    // several positions, as MySQL's `?` and SQLite's do.)
    const order = names;
    if (analysis.diagnostics.some((d) => d.code === 6) || (dialect === "postgres" && analysis.kind === "")) return emit("unknown", "unknown", inferred, order);
    const columns = analysis.columns.map((c) => `${ident(c.name)}: ${renderType(c.type, false, dialect, project.repr)}${c.nullable ? " | null" : ""}`);
    // A parameter is typed from its context (a column it is compared with or written to, a CAST);
    // one no context types is `unknown`.
    const typed = analysis.params.map((q) => [q.name, paramType(q, renderParamType(q.type, dialect, project.repr))] as const);
    const paramTypes = typed.map(([name, type]) => `${ident(name)}: ${type}`);
    // The array form names each element after its parameter (`$1` for a positional one), when
    // every name can be a label.
    const types = new Map(typed);
    const labels = order.map((n) => (/^\d+$/.test(n) ? `$${n}` : n));
    const labeled = labels.every((l) => /^[A-Za-z_$][\w$]*$/.test(l));
    const valuesType = `[${order.map((n, i) => `${labeled ? `${labels[i]}: ` : ""}${types.get(n) ?? "unknown"}`).join(", ")}]`;
    return emit(paramTypes.length ? `{ ${paramTypes.join("; ")} }` : "Record<never, never>", columns.length ? `{ ${columns.join("; ")} }` : "never", inferred, order, valuesType);
  },
  async closeProject(params) {
    const handle = (params as { projectHandle: string }).projectHandle;
    const project = projects.get(handle);
    projects.delete(handle);
    if (project) await (await project.session).close();
    return {};
  },
});
