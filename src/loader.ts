// What every build step emits for a `.sql` file: a module that carries everything the query needs
// at run time (the dialect, the statement as the engine rewrote it for the database, the binding
// order, the row count the analysis proved), so that the running code reads no file and asks no
// database. The analysis comes from `src/analyze.ts`: `.porg/` when it holds the statement for the
// current schema, else the database. A build that has neither fails, because a module built
// without the analysis would not do what its types say.
import { dirname } from "node:path";
import { Session } from "./analyze.ts";

export type SqlModule = {
  code: string;
  /** The `.porg/` entry the module was built from. A bundler watches it, so that the module is built again when the type check analyzes the statement anew (after a migration). */
  entry: string;
};

export async function sqlModule(path: string, sql: string): Promise<SqlModule> {
  const session = await Session.for(dirname(path));
  const outcome = await session.analyze(sql);
  if (!outcome.ok) throw new Error(`${path}: ${outcome.message}`);
  const { analysis } = outcome;
  let params: string[];
  try {
    params = session.lex(sql).params;
  } catch (e) {
    throw new Error(`${path}: ${(e as Error).message}`);
  }
  const code = [
    `import { defineQuery } from "porg";`,
    `export const sql = ${JSON.stringify(sql)};`,
    `export const params = ${JSON.stringify(params)};`,
    `export default defineQuery({ dialect: ${JSON.stringify(session.project.dialect)}, sql, params, cardinality: ${JSON.stringify(analysis.cardinality)}, text: ${JSON.stringify(analysis.rewritten.sql)}, order: ${JSON.stringify(analysis.rewritten.order)} });`,
    ``,
  ].join("\n");
  return { code, entry: outcome.entry };
}
