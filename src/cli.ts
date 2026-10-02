#!/usr/bin/env node
// `porg generate`: analyzes every `.sql` file under the given directories into `.porg/`, the cache
// that the type check and the build steps share, and prints one line per file. A build that runs
// where the database cannot be reached (a CI job, a container build) runs on what it wrote; where
// the database can be reached, it only saves the build the round trips.
import { existsSync, readdirSync, readFileSync, statSync } from "node:fs";
import { dirname, join, relative, resolve } from "node:path";
import { Session } from "./analyze.ts";

const USAGE = `Usage: porg generate [paths...] [--force]

Analyzes every .sql file under the paths (by default the current directory) into .porg/: the
cache that the type check and the build steps read. A file that is not one query (a migration)
is listed and skipped. --force analyzes each file against the database even when .porg/ holds it.`;

// Directories that hold no query of the project's own; a directory whose name starts with a dot
// (.git, .porg, .next) is skipped as well.
const SKIP = new Set(["node_modules", "dist", "build", "out", "target", "coverage"]);

type Row =
  | { file: string; status: "analyzed" | "cached"; cardinality: string; params: string; ms: number }
  | { file: string; status: "error" | "skipped"; detail: string };

const shown = (path: string) => relative(process.cwd(), path) || ".";

function* sqlFiles(dir: string): Generator<string> {
  for (const e of readdirSync(dir, { withFileTypes: true })) {
    if (e.name.startsWith(".")) continue;
    const full = join(dir, e.name);
    if (e.isDirectory()) {
      if (!SKIP.has(e.name)) yield* sqlFiles(full);
    } else if (e.isFile() && e.name.endsWith(".sql")) yield full;
  }
}

/** 1-based line and column of a UTF-8 byte offset, as an editor shows them. */
function position(text: string, offset: number): string {
  const lines = Buffer.from(text, "utf8").subarray(0, offset).toString("utf8").split("\n");
  return `${lines.length}:${lines.at(-1)!.length + 1}`;
}

// A file that holds DDL or several statements is not a query: a migration or a schema file, which
// nothing imports. The engine says so (or the server refuses to prepare several commands).
const NOT_A_QUERY = /^only SELECT, .* statements are typed|^expected exactly one statement|^cannot insert multiple commands into a prepared statement/;

async function generate(args: string[]): Promise<number> {
  const force = args.includes("--force");
  const unknown = args.filter((a) => a.startsWith("-") && a !== "--force");
  if (unknown.length) {
    console.error(`porg generate: unknown option ${unknown.join(", ")}\n\n${USAGE}`);
    return 2;
  }
  const paths = args.filter((a) => !a.startsWith("-"));
  const started = performance.now();

  const files = new Set<string>();
  for (const arg of paths.length ? paths : ["."]) {
    const full = resolve(arg);
    if (!existsSync(full)) {
      console.error(`porg generate: ${arg} does not exist.`);
      return 2;
    }
    if (statSync(full).isDirectory()) for (const file of sqlFiles(full)) files.add(file);
    else if (full.endsWith(".sql")) files.add(full);
    else {
      console.error(`porg generate: ${arg} is not a .sql file or a directory.`);
      return 2;
    }
  }

  // One project per `.porg.yaml` (or package): each has its database and its `.porg/`.
  const projects = new Map<Session, string[]>();
  for (const file of [...files].sort()) {
    const session = await Session.for(dirname(file));
    projects.set(session, [...(projects.get(session) ?? []), file]);
  }

  const count = { analyzed: 0, cached: 0, skipped: 0, error: 0 };
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
      const notAQuery = analysis.diagnostics.find((d) => NOT_A_QUERY.test(d.message));
      if (notAQuery) {
        rows.push({ file, status: "skipped", detail: `not a query: ${notAQuery.message.replace(/ \(\w+\)$/, "")}` });
        continue;
      }
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

  const total = files.size;
  const parts = (["analyzed", "cached", "skipped", "error"] as const).filter((k) => count[k]).map((k) => `${count[k]} ${k === "error" ? (count[k] === 1 ? "error" : "errors") : k}`);
  const seconds = ((performance.now() - started) / 1000).toFixed(1);
  console.log(total ? `${total} ${total === 1 ? "file" : "files"} in ${projects.size} ${projects.size === 1 ? "project" : "projects"}: ${parts.join(", ")} (${seconds} s)` : `No .sql files under ${(paths.length ? paths : ["."]).join(", ")}.`);
  return count.error ? 1 : 0;
}

const [command, ...rest] = process.argv.slice(2);
if (command === "generate") process.exitCode = await generate(rest);
else if (command === undefined || command === "help" || command === "--help" || command === "-h") console.log(USAGE);
else {
  console.error(`porg: unknown command ${command}\n\n${USAGE}`);
  process.exitCode = 2;
}
