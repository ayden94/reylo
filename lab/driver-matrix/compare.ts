// Compares the rows of the jobs that ran last (.out/*.jsonl, as `node runner.ts --only …` leaves
// them) with RESULTS.md, row for row: the same package, version, target, client shape and
// scenario must have the same result. The runtimes' versions are not compared (a row of node:sqlite,
// bun:sqlite or Bun.sql is keyed without its version, which is the runtime's), nor the error and
// detail texts, which carry connection ids and the drivers' wording. Rows of jobs that did not run
// are counted, not compared. Exit 1 on any difference.
//
//   node compare.ts
import { existsSync, readdirSync, readFileSync } from "node:fs";
import { join } from "node:path";

const root = import.meta.dirname;
const out = join(root, ".out");

const cells = (line: string) => line.trim().replace(/^\|/, "").replace(/\|$/, "").split("|").map((c) => c.trim());
// RESULTS.md was written before the rename of 2026-10-09 and names the package dotsql in its
// scenarios, as the run printed them then.
const named = (scenario: string) => scenario.replace(/\s+/g, " ").trim().replace(/\bdotsql\b/g, "reylo");
// The runtime is the version of node:sqlite, bun:sqlite and Bun.sql, so those are keyed without it.
const job = (pkgAtVersion: string) => (/^(node:sqlite|bun:sqlite|Bun\.sql \(\w+\))@/.test(pkgAtVersion) ? pkgAtVersion.replace(/@.*$/, "") : pkgAtVersion);

function table(md: string, heading: string): string[][] {
  const lines = md.split("\n");
  const start = lines.indexOf(heading);
  if (start === -1) throw new Error(`RESULTS.md has no ${heading}`);
  const rows: string[][] = [];
  let seenRule = false;
  for (const line of lines.slice(start + 1)) {
    if (line.startsWith("## ")) break;
    if (!line.startsWith("|")) continue;
    if (line.startsWith("| ---")) seenRule = true;
    else if (seenRule) rows.push(cells(line));
  }
  return rows;
}

// Recorded: group (job and target) → key → results.
const md = readFileSync(join(root, "RESULTS.md"), "utf8");
const recorded = new Map<string, Map<string, string[]>>();
const put = (map: Map<string, Map<string, string[]>>, group: string, key: string, status: string) => {
  const g = map.get(group) ?? new Map<string, string[]>();
  map.set(group, g);
  g.set(key, [...(g.get(key) ?? []), status]);
};
for (const [pkg, , target, shape, scenario, result] of table(md, "## Every row")) put(recorded, `${job(pkg!)} [${target}]`, `${shape} :: ${named(scenario!)}`, result!.toLowerCase());
for (const [control, shape, scenario, result] of table(md, "## Failing controls")) put(recorded, `${control} [control]`, `${shape} :: ${named(scenario!)}`, result!.toLowerCase());

// This run.
const ran = new Map<string, Map<string, string[]>>();
if (!existsSync(out)) throw new Error("no .out/: run runner.ts first");
for (const file of readdirSync(out).filter((f) => f.endsWith(".jsonl"))) {
  const control = file.startsWith("control_");
  for (const line of readFileSync(join(out, file), "utf8").split("\n").filter(Boolean)) {
    const r = JSON.parse(line);
    if (r.done) continue;
    const scenario = named(String(r.scenario));
    if (control) put(ran, `${r.pkg}/${r.version} [control]`, `${r.shape} :: ${scenario}`, r.status);
    else put(ran, `${job(`${r.pkg}@${r.version}`)} [${r.target}]`, `${r.shape} :: ${scenario}`, r.status);
  }
}

let compared = 0;
let differences = 0;
for (const [group, rows] of ran) {
  const want = recorded.get(group);
  if (!want) {
    console.log(`not in RESULTS.md: ${group}`);
    differences++;
    continue;
  }
  for (const key of new Set([...rows.keys(), ...want.keys()])) {
    const got = (rows.get(key) ?? []).sort().join(", ") || "(no row)";
    const expected = (want.get(key) ?? []).sort().join(", ") || "(no row)";
    compared++;
    if (got !== expected) {
      differences++;
      console.log(`${group} ${key}: RESULTS.md ${expected}, this run ${got}`);
    }
  }
}
const skipped = [...recorded.keys()].filter((g) => !ran.has(g));
console.log(`${compared} rows compared over ${ran.size} jobs, ${differences} different; ${skipped.length} recorded jobs did not run${skipped.length ? `: ${skipped.join("; ")}` : ""}`);
process.exit(differences ? 1 : 0);
