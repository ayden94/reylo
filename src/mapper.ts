// Experimental content mapper. It emits a typed module for each .sql file and
// reports what tsc hands the process, so lab projects can read it back through
// the type checker. Real SQL analysis comes later; for now Row types come from
// an optional lando.schema.json next to the tsconfig.
import { appendFileSync, existsSync, readFileSync } from "node:fs";
import { createHash } from "node:crypto";
import { dirname, join } from "node:path";
import { serve } from "./rpc.ts";

const LOG = process.env["LANDO_MAPPER_LOG"];
function log(event: string, data: unknown) {
  if (LOG) appendFileSync(LOG, JSON.stringify({ at: Date.now(), event, data }) + "\n");
}

type Project = {
  projectDir: string;
  options: unknown;
  schemaPath: string;
  schema: Record<string, Record<string, string>> | null;
  dotenv: Record<string, string>;
};
const projects = new Map<string, Project>();

function readSchema(path: string) {
  return existsSync(path) ? (JSON.parse(readFileSync(path, "utf8")) as Project["schema"]) : null;
}

function readDotenv(dir: string): Record<string, string> {
  const path = join(dir, ".env");
  if (!existsSync(path)) return {};
  const out: Record<string, string> = {};
  for (const line of readFileSync(path, "utf8").split("\n")) {
    const m = /^\s*([A-Za-z_][A-Za-z0-9_]*)\s*=\s*(.*)\s*$/.exec(line);
    if (m) out[m[1]!] = m[2]!.replace(/^["']|["']$/g, "");
  }
  return out;
}

serve({
  initialize(params) {
    log("initialize", params);
    return { positionEncoding: "utf-8", diagnosticSource: "lando" };
  },
  openProject(params) {
    const p = params as { projectHandle: string; options?: unknown; configFileName: string; compilerOptions: unknown };
    const projectDir = dirname(p.configFileName);
    const schemaPath = join(projectDir, "lando.schema.json");
    const project: Project = {
      projectDir,
      options: p.options,
      schemaPath,
      schema: readSchema(schemaPath),
      dotenv: readDotenv(projectDir),
    };
    projects.set(p.projectHandle, project);
    const configIdentity = createHash("sha1")
      .update(JSON.stringify(project.schema))
      .update(JSON.stringify(project.dotenv))
      .digest("hex");
    log("openProject", { ...p, compilerOptions: undefined, configIdentity, schemaFound: project.schema !== null });
    return { configIdentity, watchedFiles: [schemaPath, join(projectDir, ".env")] };
  },
  transform(params) {
    const p = params as { fileName: string; content: string; projectHandle: string };
    const project = projects.get(p.projectHandle)!;
    log("transform", { fileName: p.fileName, projectHandle: p.projectHandle });

    // Fake analysis: `from <table>` picks the Row type out of lando.schema.json.
    const table = /\bfrom\s+([a-z_]+)/i.exec(p.content)?.[1];
    const columns = table && project.schema ? project.schema[table] : undefined;
    const rowType = columns
      ? `{ ${Object.entries(columns).map(([k, v]) => `${k}: ${v}`).join("; ")} }`
      : "unknown";
    const diagnostics =
      table && project.schema && !columns
        ? [{ messageText: `relation "${table}" does not exist in lando.schema.json`, start: p.content.indexOf(table), length: table.length, code: 1 }]
        : [];

    const probe = {
      env: { LANDO_PROBE: process.env["LANDO_PROBE"] ?? null, DATABASE_URL: process.env["DATABASE_URL"] ?? null },
      dotenv: project.dotenv,
      options: project.options,
      schemaFound: project.schema !== null,
    };
    const text = [
      `import type { Query } from "lando";`,
      `export declare const sql: ${JSON.stringify(p.content)};`,
      `export type Row = ${rowType};`,
      `export type Params = [id: number];`,
      `export declare const probe: ${JSON.stringify(probe)};`,
      `declare const query: Query<Params, Row>;`,
      `export default query;`,
      ``,
    ].join("\n");
    return { text, extension: ".ts", diagnostics };
  },
  closeProject(params) {
    log("closeProject", params);
    projects.delete((params as { projectHandle: string }).projectHandle);
    return {};
  },
});
