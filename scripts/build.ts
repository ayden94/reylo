// Builds the publishable package into dist/: the sources compiled to JavaScript with their
// declarations, the engine's WebAssembly, the README, and a package.json whose exports and
// content mapper name the compiled files. Node strips no types under node_modules, so an
// installed copy that ships TypeScript fails to load in the mapper, the bundler adapters and at
// run time. The repository's own package.json keeps pointing at the sources, so the tests and
// the labs run them without a build; `npm publish dist` publishes the build, and the root is
// private so that it cannot be published by mistake. dist/ keeps the repository's layout
// (src/, engine/), so `src/engine.ts` finds `../engine/porg_engine.wasm` in both.
import { execFileSync } from "node:child_process";
import { copyFileSync, mkdirSync, readdirSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { join, relative } from "node:path";

const root = join(import.meta.dirname, "..");
const dist = join(root, "dist");
const js = (path: string) => path.replace(/\.ts$/, ".js");

rmSync(dist, { recursive: true, force: true });
execFileSync(join(root, "node_modules/.bin/tsc"), ["-p", join(root, "tsconfig.build.json")], { stdio: "inherit" });
mkdirSync(join(dist, "engine"));
copyFileSync(join(root, "engine/porg_engine.wasm"), join(dist, "engine/porg_engine.wasm"));
copyFileSync(join(root, "README.md"), join(dist, "README.md"));

const pkg = JSON.parse(readFileSync(join(root, "package.json"), "utf8"));
const { private: _private, scripts: _scripts, devDependencies: _dev, exports, typescript, ...rest } = pkg;
const built = {
  ...rest,
  exports: Object.fromEntries(Object.entries(exports as Record<string, string>).map(([key, path]) => [key, { types: path.replace(/\.ts$/, ".d.ts"), default: js(path) }])),
  typescript: { ...typescript, contentMapper: { ...typescript.contentMapper, exec: (typescript.contentMapper.exec as string[]).map(js) } },
};
writeFileSync(join(dist, "package.json"), JSON.stringify(built, null, 2) + "\n");

// Nothing TypeScript ships but declarations.
const files = readdirSync(dist, { recursive: true, encoding: "utf8" });
const source = files.filter((f) => /\.(c|m)?ts$/.test(f) && !/\.d\.(c|m)?ts$/.test(f));
if (source.length) throw new Error(`dist/ holds TypeScript sources: ${source.join(", ")}`);
console.log(`dist/: ${files.length} files, package ${built.name}@${built.version}`);
