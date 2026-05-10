// `lando.config.ts` finder + loader. Uses `tsx`'s programmatic API
// (`tsImport`) so the CLI doesn't need a precompile step or a separate
// `lando build` command. The user just writes TypeScript; we evaluate
// it on demand.
//
// Failures are returned as structured errors (one or more `ConfigError`
// values) — never thrown — so the CLI can format them coherently and
// `--json` output stays predictable.

import { existsSync } from 'node:fs';
import { dirname, isAbsolute, resolve } from 'node:path';
import { pathToFileURL } from 'node:url';
import { tsImport } from 'tsx/esm/api';
import { CONFIG_ERROR_CODES, type ConfigError } from './errors.ts';
import { findProjectRoot, readProjectMeta, type ProjectMeta } from './project.ts';
import { validateConfig } from './validate.ts';
import type { LandoConfig } from './types.ts';

export type LoadConfigOptions = {
  /** Working directory to start the search from. Defaults to `process.cwd()`. */
  cwd?: string;
  /**
   * Explicit path to the config file. When set, the upward search is
   * skipped. Relative paths are resolved against `cwd`.
   */
  path?: string;
};

export type LoadConfigResult =
  | {
      ok: true;
      config: LandoConfig;
      configPath: string;
      meta: ProjectMeta;
    }
  | {
      ok: false;
      errors: ConfigError[];
      /** Set when we got far enough to know the file path. */
      configPath?: string;
    };

const CONFIG_FILENAME = 'lando.config.ts';

/**
 * Walks upward from `cwd` looking for `lando.config.ts`. Stops at the
 * filesystem root. Returns the absolute path or `undefined`.
 */
export function findConfig(cwd: string): string | undefined {
  let current = resolve(cwd);
  while (true) {
    const candidate = resolve(current, CONFIG_FILENAME);
    if (existsSync(candidate)) return candidate;
    const parent = dirname(current);
    if (parent === current) return undefined;
    current = parent;
  }
}

/**
 * Locates and loads `lando.config.ts`. Returns the validated config plus
 * the project metadata (artifact directory, project root). Bails with
 * structured errors at every stage that can fail.
 */
export async function loadConfig(opts: LoadConfigOptions = {}): Promise<LoadConfigResult> {
  const cwd = resolve(opts.cwd ?? process.cwd());

  const configPath =
    opts.path !== undefined
      ? isAbsolute(opts.path)
        ? opts.path
        : resolve(cwd, opts.path)
      : findConfig(cwd);

  if (configPath === undefined) {
    return {
      ok: false,
      errors: [
        {
          code: CONFIG_ERROR_CODES.L_CONFIG_NOT_FOUND,
          message: `no ${CONFIG_FILENAME} found at or above ${cwd}`,
          path: cwd,
        },
      ],
    };
  }
  if (!existsSync(configPath)) {
    return {
      ok: false,
      configPath,
      errors: [
        {
          code: CONFIG_ERROR_CODES.L_CONFIG_NOT_FOUND,
          message: `${configPath} does not exist`,
          path: configPath,
        },
      ],
    };
  }

  let mod: unknown;
  try {
    mod = await tsImport(pathToFileURL(configPath).href, import.meta.url);
  } catch (cause) {
    return {
      ok: false,
      configPath,
      errors: [
        {
          code: CONFIG_ERROR_CODES.L_CONFIG_LOAD_FAILED,
          message: `failed to load ${configPath}: ${(cause as Error).message}`,
          path: configPath,
          cause,
        },
      ],
    };
  }

  const defaultExport = (mod as Record<string, unknown>)['default'];
  if (defaultExport === undefined) {
    return {
      ok: false,
      configPath,
      errors: [
        {
          code: CONFIG_ERROR_CODES.L_CONFIG_NO_DEFAULT_EXPORT,
          message: `${configPath} has no default export; expected \`export default { ... } satisfies LandoConfig\``,
          path: configPath,
        },
      ],
    };
  }

  const validation = validateConfig(defaultExport);
  if (!validation.ok) {
    return { ok: false, configPath, errors: validation.errors };
  }

  const projectRoot = findProjectRoot(dirname(configPath)) ?? dirname(configPath);
  const metaResult = readProjectMeta(projectRoot);
  if (!metaResult.ok) {
    return { ok: false, configPath, errors: metaResult.errors };
  }

  return {
    ok: true,
    configPath,
    config: validation.config,
    meta: metaResult.meta,
  };
}
