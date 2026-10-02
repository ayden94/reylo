// The project file, `.porg.yaml`, and the connection it names. The file sits beside the project's
// `package.json` (the nearest one at or above the tsconfig's directory; `PORG_CONFIG` names
// another file), is parsed by the engine so that every Porg tool reads the same YAML, and is
// resolved here against the environment: the process environment first, then the `.env` files the
// project file names under `env` (by default `.env.local`, then `.env`, beside it), and a `${VAR}`
// in a URL is replaced from that. The URL's scheme names the dialect. A project without the file
// behaves as if it held `url: ${DATABASE_URL}`.
import { existsSync, readFileSync } from "node:fs";
import { dirname, isAbsolute, join, relative, resolve } from "node:path";
import type { Engine } from "./engine.ts";
import { defaultReprs, type Repr } from "./render.ts";

export type Dialect = "postgres" | "sqlite" | "mysql";

/** What the engine reads out of `.porg.yaml`: the keys the library uses. */
export type ProjectFile = {
  url?: string;
  direct_url?: string;
  env?: string[];
  strict?: boolean;
  types: { dates?: Repr["dates"]; bigint?: Repr["bigint"] };
  cli?: Cli;
};

/** The files `porg generate` analyzes, as globs relative to the project file's directory. */
export type Cli = { include?: string[]; exclude?: string[] };

export type ResolvedProject = {
  /** The project file, when there is one. */
  file: string | undefined;
  /** The directory a relative path in the project resolves against: the package's. */
  root: string;
  dialect: Dialect;
  repr: Repr;
  /** Whether a parameter the statement does not type is an error rather than `unknown`. */
  strict: boolean;
  /** Which files `porg generate` analyzes: `include` replaces every `.sql` file, `exclude` adds to what is left out. */
  cli: Cli;
  /** The connection the type check uses: `direct_url` when the file names one, else `url`. */
  url: string | undefined;
  /** Where the URL came from, for a diagnostic: never the URL itself. */
  urlSource: string;
  /** The files a change to which changes the project: the project file and the `.env` files. */
  watched: string[];
  /** Why the project cannot be used, already placed (`.porg.yaml:3:1: ...`); every transform reports it. */
  error: string | undefined;
};

export const PROJECT_FILE = ".porg.yaml";
const DEFAULT_ENV_FILES = [".env.local", ".env"];

/** The package directory: the nearest one at or above `from` that holds a `package.json`, else `from`. */
export function packageDir(from: string): string {
  for (let dir = resolve(from); ; dir = dirname(dir)) {
    if (existsSync(join(dir, "package.json"))) return dir;
    if (dirname(dir) === dir) return resolve(from);
  }
}

/** The project file for a project directory: `PORG_CONFIG`, or `.porg.yaml` beside the package's `package.json`. */
export function findProjectFile(from: string, env: Record<string, string | undefined> = process.env): string | undefined {
  if (env.PORG_CONFIG) return resolve(env.PORG_CONFIG);
  const candidate = join(packageDir(from), PROJECT_FILE);
  return existsSync(candidate) ? candidate : undefined;
}

/** The dialect a URL's scheme names, or undefined for a scheme Porg does not read. */
export function dialectOf(url: string): Dialect | undefined {
  const scheme = /^([a-z][a-z0-9+.-]*):/i.exec(url)?.[1]?.toLowerCase();
  if (scheme === "postgres" || scheme === "postgresql") return "postgres";
  if (scheme === "mysql" || scheme === "mariadb") return "mysql";
  if (scheme === "file" || scheme === "sqlite") return "sqlite";
  return undefined;
}

export function readEnvFile(path: string): Record<string, string> {
  if (!existsSync(path)) return {};
  const out: Record<string, string> = {};
  for (const line of readFileSync(path, "utf8").split("\n")) {
    const m = /^\s*(?:export\s+)?([A-Za-z_][A-Za-z0-9_]*)\s*=\s*(.*?)\s*$/.exec(line);
    if (m) out[m[1]!] = m[2]!.replace(/^(["'])(.*)\1$/, "$2");
  }
  return out;
}

/** 1-based line and column of a byte offset, for a message that names the place in the file. */
function place(text: string, offset: number): string {
  const before = Buffer.from(text, "utf8").subarray(0, offset).toString("utf8");
  const lines = before.split("\n");
  return `${lines.length}:${lines.at(-1)!.length + 1}`;
}

export function resolveProject(engine: Engine, projectDir: string, env: Record<string, string | undefined> = process.env): ResolvedProject {
  const file = findProjectFile(projectDir, env);
  const root = file ? dirname(file) : packageDir(projectDir);
  const shown = (path: string) => (isAbsolute(path) ? relative(projectDir, path) || path : path);
  let config: ProjectFile = { types: {} };
  let error: string | undefined;
  if (file) {
    let text: string | undefined;
    try {
      text = readFileSync(file, "utf8");
    } catch (e) {
      error = `Cannot read ${shown(file)}: ${(e as Error).message}`;
    }
    if (text !== undefined) {
      const r = engine.call({ op: "config", text });
      if (r.status === "config") config = r.config as ProjectFile;
      else if (r.status === "configError") error = `${shown(file)}:${place(text, r.error.start)}: ${r.error.message}`;
      else error = `${shown(file)}: ${r.status === "error" ? r.message : `unexpected ${r.status}`}`;
    }
  }
  const envFiles = (config.env ?? DEFAULT_ENV_FILES).map((f) => resolve(root, f));
  // The process environment wins; among the files, the first that sets a variable does.
  const lookup = (name: string): { value: string; source: string } | undefined => {
    if (env[name] !== undefined) return { value: env[name]!, source: `${name} in the environment` };
    for (const path of envFiles) {
      const value = readEnvFile(path)[name];
      if (value !== undefined) return { value, source: `${name} in ${shown(path)}` };
    }
    return undefined;
  };
  let url: string | undefined;
  let urlSource = "DATABASE_URL";
  const key = config.direct_url !== undefined ? "direct_url" : "url";
  const written = config.direct_url ?? config.url;
  if (written !== undefined) {
    const missing: string[] = [];
    const sources: string[] = [];
    url = written.replace(/\$\{([A-Za-z_][A-Za-z0-9_]*)\}/g, (_, name: string) => {
      const hit = lookup(name);
      if (!hit) missing.push(name);
      else sources.push(hit.source);
      return hit?.value ?? "";
    });
    urlSource = `"${key}" in ${shown(file!)}${sources.length ? `, ${sources.join(", ")}` : ""}`;
    if (missing.length) {
      error ??= `${shown(file!)}: "${key}" names ${missing.map((m) => `\${${m}}`).join(", ")}, which neither the environment nor ${envFiles.map(shown).join(" nor ")} sets.`;
      url = undefined;
    }
  } else {
    const hit = lookup("DATABASE_URL");
    url = hit?.value;
    if (hit) urlSource = hit.source;
  }
  const dialect = url ? dialectOf(url) : "postgres";
  if (url && !dialect) error ??= `Cannot tell the database from the URL (${urlSource}): Porg reads postgres://, postgresql://, mysql://, mariadb://, file: and sqlite: URLs.`;
  const defaults = defaultReprs[dialect ?? "postgres"];
  const repr: Repr = { dates: config.types.dates ?? defaults.dates, bigint: config.types.bigint ?? defaults.bigint };
  return { file, root, dialect: dialect ?? "postgres", repr, strict: config.strict ?? false, cli: config.cli ?? {}, url, urlSource, watched: [...(file ? [file] : [join(root, PROJECT_FILE)]), ...envFiles], error };
}
