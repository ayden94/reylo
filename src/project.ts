// The project file, `.porg.toml`, and the connection it names. The file is found by walking up
// from the tsconfig's directory (`PORG_CONFIG` names one instead), parsed by the engine so that
// every Porg tool reads the same TOML, and resolved here against the environment: the process
// environment first, then the `.env` files the project file names (by default `.env.local`, then
// `.env`), and a `${VAR}` in `url` is replaced from that. A project without the file behaves as if
// it held `url = "${DATABASE_URL}"`.
import { existsSync, readFileSync } from "node:fs";
import { dirname, isAbsolute, join, relative, resolve } from "node:path";
import type { Engine } from "./engine.ts";
import { defaultRepr, type Repr } from "./render.ts";

export type Dialect = "postgres" | "sqlite" | "mysql";

/** What the engine reads out of `.porg.toml`: the keys the library uses. */
export type ProjectFile = {
  dialect?: Dialect;
  url?: string;
  env_files?: string[];
  types: { dates?: Repr["dates"]; bigint?: Repr["bigint"] };
};

export type ResolvedProject = {
  /** The project file, when there is one. */
  file: string | undefined;
  /** The directory a relative path in the project resolves against: the file's, or the tsconfig's. */
  root: string;
  dialect: Dialect;
  repr: Repr;
  url: string | undefined;
  /** Where the URL came from, for a diagnostic: never the URL itself. */
  urlSource: string;
  /** The files a change to which changes the project: the project file and the `.env` files. */
  watched: string[];
  /** Why the project cannot be used, already placed (`.porg.toml:3:1: ...`); every transform reports it. */
  error: string | undefined;
};

export const PROJECT_FILE = ".porg.toml";
const DEFAULT_ENV_FILES = [".env.local", ".env"];

/** The project file for a project directory: `PORG_CONFIG`, or the first `.porg.toml` walking up. */
export function findProjectFile(from: string, env: Record<string, string | undefined> = process.env): string | undefined {
  if (env.PORG_CONFIG) return resolve(env.PORG_CONFIG);
  for (let dir = resolve(from); ; dir = dirname(dir)) {
    const candidate = join(dir, PROJECT_FILE);
    if (existsSync(candidate)) return candidate;
    if (dirname(dir) === dir) return undefined;
  }
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
  const root = file ? dirname(file) : projectDir;
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
  const envFiles = (config.env_files ?? DEFAULT_ENV_FILES).map((f) => resolve(root, f));
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
  if (config.url !== undefined) {
    const missing: string[] = [];
    const sources: string[] = [];
    url = config.url.replace(/\$\{([A-Za-z_][A-Za-z0-9_]*)\}/g, (_, name: string) => {
      const hit = lookup(name);
      if (!hit) missing.push(name);
      else sources.push(hit.source);
      return hit?.value ?? "";
    });
    urlSource = `"url" in ${shown(file!)}${sources.length ? `, ${sources.join(", ")}` : ""}`;
    if (missing.length) {
      error ??= `${shown(file!)}: "url" names ${missing.map((m) => `\${${m}}`).join(", ")}, which neither the environment nor ${envFiles.map(shown).join(" nor ")} sets.`;
      url = undefined;
    }
  } else {
    const hit = lookup("DATABASE_URL");
    url = hit?.value;
    if (hit) urlSource = hit.source;
  }
  const dialect = config.dialect ?? (url && /^(file:|sqlite:)/.test(url) ? "sqlite" : url && /^(mysql|mariadb):/.test(url) ? "mysql" : "postgres");
  const repr: Repr = { dates: config.types.dates ?? defaultRepr.dates, bigint: config.types.bigint ?? defaultRepr.bigint };
  return { file, root, dialect, repr, url, urlSource, watched: [...(file ? [file] : [join(projectDir, PROJECT_FILE)]), ...envFiles], error };
}
