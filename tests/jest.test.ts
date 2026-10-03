import { test } from "node:test";
import assert from "node:assert/strict";
import { spawnSync } from "node:child_process";
import { existsSync, mkdtempSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { fileURLToPath } from "node:url";

// Jest and Vitest are lab/jest's dependencies, never the package's own.
const lab = fileURLToPath(new URL("../lab/jest", import.meta.url));
const skip = !existsSync(`${lab}/node_modules/jest`) && "lab/jest is not installed";

function run(bin: string, args: string[], nodeOptions = "") {
  const r = spawnSync(process.execPath, [`${lab}/node_modules/${bin}`, ...args], { cwd: lab, encoding: "utf8", env: { ...process.env, NODE_OPTIONS: `--no-warnings ${nodeOptions}` } });
  return { status: r.status, output: r.stdout + r.stderr };
}

const cache = mkdtempSync(join(tmpdir(), "motherload-jest-"));
const jest = (args: string[], nodeOptions?: string) => run("jest/bin/jest.js", ["--ci", `--cacheDirectory=${cache}`, ...args], nodeOptions);

test("setup", { skip }, () => {
  assert.equal(spawnSync(process.execPath, ["setup.mjs"], { cwd: lab }).status, 0);
});

test("Jest's runtime never asks Node's loader: the motherload/register preload leaves a .sql file to Jest, which reads it as JavaScript", { skip }, () => {
  const r = jest(["--config", JSON.stringify({ watchman: false, testPathIgnorePatterns: ["/node_modules/", "/probe/"], extensionsToTreatAsEsm: [".sql"] })], "--experimental-vm-modules --import motherload/register");
  assert.equal(r.status, 1, r.output);
  assert.match(r.output, /SyntaxError: Unexpected identifier 'id'/);
});

test("motherload/jest: a CommonJS run, with motherload's runtime compiled to CommonJS by babel-jest", { skip }, () => {
  const r = jest(["-c", "jest.cjs.config.cjs", "query.test.cjs"]);
  assert.equal(r.status, 0, r.output);
  assert.match(r.output, /Tests:\s+1 passed, 1 total/);
});

test("motherload/jest: an ESM run (--experimental-vm-modules), a CommonJS and an ESM test", { skip }, () => {
  const r = jest(["-c", "jest.esm.config.cjs"], "--experimental-vm-modules");
  assert.equal(r.status, 0, r.output);
  assert.match(r.output, /Tests:\s+2 passed, 2 total/);
});

test("Vitest needs nothing beyond motherload/vite", { skip }, () => {
  const r = run("vitest/vitest.mjs", ["run"]);
  assert.equal(r.status, 0, r.output);
  assert.match(r.output, /Tests\s+1 passed/);
});
