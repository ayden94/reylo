// Runs the official JSON Schema Test Suite (draft 2020-12) over the run time's validator,
// src/json-schema.ts, which checks a typed JSON parameter's value and a `json('…')` member's
// before the statement runs (wiki/typed-json-params.md). The suite is fetched into `.suite/` at the
// commit below. The keywords the profile refuses are skipped and counted (references, the dynamic
// keywords, `unevaluatedItems`, a schema in `unevaluatedProperties`); `format` is asserted, as the
// record decides, so the suite's optional format files run and its annotation-only `format.json`
// does not. It exits 1 when any test fails.
//
//   env -u NODE_COMPAT nub run.ts
import { execFileSync } from "node:child_process";
import { existsSync, mkdirSync, readdirSync, readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { validate } from "../../src/json-schema.ts";

const COMMIT = "5571928";
const here = dirname(fileURLToPath(import.meta.url));
const suite = join(here, ".suite");
if (!existsSync(join(suite, "tests"))) {
  mkdirSync(suite, { recursive: true });
  execFileSync("git", ["clone", "-q", "https://github.com/json-schema-org/JSON-Schema-Test-Suite.git", suite]);
}
execFileSync("git", ["-C", suite, "checkout", "-q", COMMIT]);

const REFUSED = ["$ref", "$defs", "definitions", "$dynamicRef", "$dynamicAnchor", "$recursiveRef", "$recursiveAnchor", "$anchor", "$id", "$vocabulary", "unevaluatedItems", "contentSchema"];
function refused(s: unknown): boolean {
  if (s === null || typeof s !== "object") return false;
  if (Array.isArray(s)) return s.some(refused);
  return Object.entries(s).some(([k, v]) => REFUSED.includes(k) || (k === "unevaluatedProperties" && typeof v !== "boolean") || (k === "$schema" && v !== "https://json-schema.org/draft/2020-12/schema") || refused(v));
}
// The files of keywords the profile refuses, and of behaviour Reylo decides otherwise.
const SKIP = new Set(["ref.json", "refRemote.json", "anchor.json", "defs.json", "dynamicRef.json", "vocabulary.json", "infinite-loop-detection.json", "unevaluatedItems.json", "format.json", "content.json", "default.json"]);
const FORMATS = new Set(["date-time.json", "date.json", "time.json", "email.json", "ipv4.json", "ipv6.json", "uuid.json", "hostname.json", "uri.json", "regex.json", "unknown.json"]);
let pass = 0;
let skipped = 0;
const failures: string[] = [];
function run(file: string, label: string) {
  for (const group of JSON.parse(readFileSync(file, "utf8")) as { description: string; schema: unknown; tests: { description: string; data: unknown; valid: boolean }[] }[]) {
    if (refused(group.schema)) {
      skipped += group.tests.length;
      continue;
    }
    for (const t of group.tests) {
      const valid = validate(group.schema as boolean | Record<string, unknown>, t.data) === undefined;
      if (valid === t.valid) pass++;
      else failures.push(`${label}: ${group.description}: ${t.description} (expected ${t.valid ? "valid" : "invalid"})`);
    }
  }
}
const dir = join(suite, "tests/draft2020-12");
for (const f of readdirSync(dir).filter((f) => f.endsWith(".json") && !SKIP.has(f))) run(join(dir, f), f);
for (const f of readdirSync(join(dir, "optional/format")).filter((f) => FORMATS.has(f))) run(join(dir, "optional/format", f), `format/${f}`);
for (const f of ["bignum.json", "ecmascript-regex.json", "non-bmp-regex.json", "float-overflow.json"]) run(join(dir, "optional", f), `optional/${f}`);
console.log(`pass ${pass}, fail ${failures.length}, skipped (the profile refuses their keywords) ${skipped}`);
for (const f of failures) console.log(`FAIL ${f}`);
process.exitCode = failures.length ? 1 : 0;
