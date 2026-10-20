// What every build step emits for a `.sql` file: a module that carries everything the query needs
// at run time (the dialect, the statement as the engine rewrote it for the database, the binding
// order, each parameter's type, the row count the analysis proved), so that the running code reads no file and asks no
// database. The analysis comes from `src/analyze.ts`: `.reylo/` when it holds the statement for the
// current schema, else the database. A build that has neither fails, because a module built
// without the analysis would not do what its types say.
import { dirname, relative } from "node:path";
import { Session } from "./analyze.ts";
import { querySchemas } from "./schema.ts";

export type SqlModule = {
  code: string;
  /** The `.reylo/` entry the module was built from. A bundler watches it, so that the module is built again when the type check analyzes the statement anew (after a migration). */
  entry: string;
};

export async function sqlModule(path: string, sql: string): Promise<SqlModule> {
  const session = await Session.for(dirname(path));
  const outcome = await session.analyze(sql);
  if (!outcome.ok) throw new Error(`${path}: ${outcome.message}`);
  const { analysis } = outcome;
  // A statement whose placeholders break a rule fails the build: one the engine could not read
  // (1), named and positional mixed (2), or a parameter written more than once (3), since each
  // parameter appears once so that one place types it.
  const unread = analysis.diagnostics.find((d) => d.code === 1 || d.code === 2 || d.code === 3);
  if (unread) throw new Error(`${path}: ${unread.message}`);
  let params: string[];
  if (session.project.dialect === "mysql") {
    // A MySQL session reads its strings by its sql_mode, which the analysis asked for: its
    // placeholders are the ones the server finds.
    params = analysis.params.map((p) => p.name);
  } else {
    try {
      params = session.lex(sql).params;
    } catch (e) {
      throw new Error(`${path}: ${(e as Error).message}`);
    }
  }
  // A column list that does not read, or that disagrees with the table its insert writes, fails
  // the build too.
  const descriptor = analysis.diagnostics.find((d) => d.code === 20 || d.code === 21);
  if (descriptor) throw new Error(`${path}: ${descriptor.message}`);
  // Each parameter's type, for the drivers that bind an array or a JSON value wrong (src/execute.ts).
  const inputs = Object.fromEntries(analysis.params.map((p) => [p.name, p.schema === undefined ? { type: p.type } : { type: p.type, schema: p.schema }]));
  // A structured parameter's lists, at JavaScript's offsets into the statement (the engine's are
  // UTF-8 bytes).
  const text = analysis.rewritten.sql;
  const structured = (analysis.structured ?? []).map((s) => ({ name: s.name, lists: s.lists.map((l) => ({ start: utf16(text, l.start), end: utf16(text, l.end), entries: l.entries })) }));
  const code = [
    `import { defineQuery } from "reylo";`,
    `export const sql = ${JSON.stringify(sql)};`,
    `export const params = ${JSON.stringify(params)};`,
    `export default defineQuery({ dialect: ${JSON.stringify(session.project.dialect)}, sql, params, cardinality: ${JSON.stringify(analysis.cardinality)}, text: ${JSON.stringify(text)}, order: ${JSON.stringify(analysis.rewritten.order)}, inputs: ${JSON.stringify(inputs)}${structured.length ? `, structured: ${JSON.stringify(structured)}` : ""}${analysis.fresh ? `, fresh: ${JSON.stringify(analysis.fresh)}, source: ${JSON.stringify(relative(session.project.root, path))}` : ""}, jsonSchema: ${JSON.stringify(querySchemas(analysis, session.project.dialect, session.project.repr))} });`,
    ``,
  ].join("\n");
  return { code, entry: outcome.entry };
}

/** The JavaScript offset into `text` of a UTF-8 byte offset. */
function utf16(text: string, bytes: number): number {
  return Buffer.from(text, "utf8").subarray(0, bytes).toString("utf8").length;
}
