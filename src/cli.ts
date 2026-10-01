#!/usr/bin/env node
// `porg generate`: analyzes the `.sql` files a project imports into `.porg/`, the cache that the
// type check and the build steps share, and prints one line per file. A build that runs where the
// database cannot be reached (a CI job, a container build) runs on what it wrote; where the
// database can be reached, it only saves the build the round trips.
import { existsSync, readdirSync, readFileSync, statSync } from "node:fs";
import { dirname, isAbsolute, join, relative, resolve } from "node:path";
import { Session } from "./analyze.ts";

const USAGE = `Usage: porg generate [paths...] [--force]

Analyzes every .sql file that the sources under the paths (by default the current directory)
import, and every .sql file named, into .porg/: the cache that the type check and the build steps
read. --force analyzes each file against the database even when .porg/ holds it.`;

// Directories that hold no source of the project's own; a directory whose name starts with a dot
// (.git, .porg, .next) is skipped as well.
const SKIP = new Set(["node_modules", "dist", "build", "out", "target", "coverage"]);
const SOURCE = /\.[cm]?[jt]sx?$/;
// A specifier that ends in `.sql` after `from`, `import` or `require(`: a static import or
// re-export, a side-effect import, a dynamic import, a require.
const SPECIFIER = /(?:\bfrom|\bimport|\brequire)\s*\(?\s*(["'])([^"'\n]+?\.sql)(?:\?[^"'\n]*)?\1/g;

type Row = { file: string; status: "analyzed" | "cached" | "error"; detail: string } | { file: string; status: "analyzed" | "cached"; cardinality: string; params: string; ms: number };

const shown = (path: string) => relative(process.cwd(), path) || ".";

function* sources(dir: string): Generator<string> {
  for (const e of readdirSync(dir, { withFileTypes: true })) {
    if (e.name.startsWith(".")) continue;
    const full = join(dir, e.name);
    if (e.isDirectory()) {
      if (!SKIP.has(e.name)) yield* sources(full);
    } else if (e.isFile() && SOURCE.test(e.name)) yield full;
  }
}

/** 1-based line and column of a UTF-8 byte offset, as an editor shows them. */
function position(text: string, offset: number): string {
  const lines = Buffer.from(text, "utf8").subarray(0, offset).toString("utf8").split("\n");
  return `${lines.length}:${lines.at(-1)!.length + 1}`;
}

async function generate(args: string[]): Promise<number> {
  const force = args.includes("--force");
  const unknown = args.filter((a) => a.startsWith("-") && a !== "--force");
  if (unknown.length) {
    console.error(`porg generate: unknown option ${unknown.join(", ")}\n\n${USAGE}`);
    return 2;
  }
  const paths = args.filter((a) => !a.startsWith("-"));
  const started = performance.now();

  // The `.sql` files, each with the files that import it.
  const files = new Map<string, string[]>();
  const skipped: string[] = [];
  const add = (file: string, by: string) => files.set(file, [...(files.get(file) ?? []), by]);
  const scan = (source: string) => {
    for (const m of readFileSync(source, "utf8").matchAll(SPECIFIER)) {
      const specifier = m[2]!;
      if (specifier.startsWith("./") || specifier.startsWith("../")) add(resolve(dirname(source), specifier), source);
      else if (isAbsolute(specifier)) add(specifier, source);
      else skipped.push(`"${specifier}" in ${shown(source)}`);
    }
  };
  for (const arg of paths.length ? paths : ["."]) {
    const full = resolve(arg);
    if (!existsSync(full)) {
      console.error(`porg generate: ${arg} does not exist.`);
      return 2;
    }
    if (statSync(full).isDirectory()) for (const source of sources(full)) scan(source);
    else if (full.endsWith(".sql")) add(full, full);
    else scan(full);
  }

  // One project per `.porg.yaml` (or package): each has its database and its `.porg/`.
  const projects = new Map<Session, string[]>();
  for (const file of [...files.keys()].sort()) {
    const session = await Session.for(dirname(file));
    projects.set(session, [...(projects.get(session) ?? []), file]);
  }

  const count = { analyzed: 0, cached: 0, error: 0 };
  for (const [session, list] of projects) {
    const { project } = session;
    console.log(`${shown(session.root)}: ${project.dialect} at ${session.where()} (${project.urlSource})`);
    // A project that cannot be used at all is said once, above its files.
    let unusable = project.error ?? (project.url ? undefined : `No database URL: set DATABASE_URL in the environment, .env.local or .env, or "url" in .porg.yaml.`);
    let offline = false;
    if (!unusable) {
      const schema = await session.schema();
      if (schema.fatal) unusable = schema.error;
      else if (schema.value === null) {
        offline = true;
        console.log(`  ${session.cannotConnect(schema.error)}; the files are read from .porg/ as they are.`);
      }
    }
    if (unusable) console.log(`  ${unusable}`);
    const rows: Row[] = [];
    for (const file of list) {
      if (!existsSync(file)) {
        rows.push({ file, status: "error", detail: `no such file; imported by ${files.get(file)!.map(shown).join(", ")}` });
        continue;
      }
      if (unusable) {
        rows.push({ file, status: "error", detail: "not analyzed" });
        continue;
      }
      const sql = readFileSync(file, "utf8");
      const t = performance.now();
      const outcome = await session.analyze(sql, { force });
      const ms = performance.now() - t;
      if (!outcome.ok) {
        rows.push({ file, status: "error", detail: offline ? (force ? "--force needs the database" : "not in .porg/") : outcome.message });
        continue;
      }
      const { analysis } = outcome;
      if (analysis.diagnostics.length) {
        rows.push({ file, status: "error", detail: analysis.diagnostics.map((d) => `${shown(file)}:${position(sql, d.start)}: ${d.message}`).join("\n") });
        continue;
      }
      rows.push({ file, status: outcome.source === "cache" ? "cached" : "analyzed", cardinality: analysis.cardinality, params: analysis.params.map((p) => p.name).join(", ") || "-", ms });
    }
    const width = Math.max(...rows.map((r) => shown(r.file).length));
    const paramsWidth = Math.max(0, ...rows.map((r) => ("params" in r ? r.params.length : 0)));
    for (const row of rows) {
      count[row.status]++;
      const head = `  ${row.status.padEnd(8)}  ${shown(row.file).padEnd(width)}  `;
      if ("detail" in row) {
        const [first, ...more] = row.detail.split("\n");
        console.log(`${head}${first}`);
        for (const line of more) console.log(`${"".padEnd(head.length)}${line}`);
      } else console.log(`${head}${row.cardinality.padEnd(8)}  ${row.params.padEnd(paramsWidth)}  ${String(Math.round(row.ms)).padStart(4)} ms`);
    }
  }
  for (const session of Session.opened()) await session.close();

  if (skipped.length) console.log(`Skipped ${skipped.length} ${skipped.length === 1 ? "import" : "imports"} that ${skipped.length === 1 ? "is not a relative path" : "are not relative paths"}: ${skipped.join(", ")}.`);
  const total = files.size;
  const parts = (["analyzed", "cached", "error"] as const).filter((k) => count[k]).map((k) => `${count[k]} ${k === "error" ? (count[k] === 1 ? "error" : "errors") : k}`);
  const seconds = ((performance.now() - started) / 1000).toFixed(1);
  console.log(total ? `${total} ${total === 1 ? "file" : "files"} in ${projects.size} ${projects.size === 1 ? "project" : "projects"}: ${parts.join(", ")} (${seconds} s)` : `No .sql imports found under ${(paths.length ? paths : ["."]).join(", ")}.`);
  return count.error ? 1 : 0;
}

const [command, ...rest] = process.argv.slice(2);
if (command === "generate") process.exitCode = await generate(rest);
else if (command === undefined || command === "help" || command === "--help" || command === "-h") console.log(USAGE);
else {
  console.error(`porg: unknown command ${command}\n\n${USAGE}`);
  process.exitCode = 2;
}
