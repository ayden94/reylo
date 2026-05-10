// Project conventions: where the artifact lives, where the project root
// is. Whitepaper §"Generated artifacts" specifies the
// `package.json#lando.dir` override; default is `.lando/`.

import { readFileSync } from 'node:fs';
import { dirname, isAbsolute, resolve } from 'node:path';
import { CONFIG_ERROR_CODES, type ConfigError } from './errors.ts';

export type ProjectMeta = {
  /** Absolute path to the directory containing `package.json`. */
  readonly projectRoot: string;
  /** Absolute path to the artifact directory (default `<root>/.lando`). */
  readonly artifactDir: string;
};

export type ReadProjectMetaResult =
  | { ok: true; meta: ProjectMeta }
  | { ok: false; errors: ConfigError[] };

/**
 * Reads project conventions from `<projectRoot>/package.json`. Tolerant
 * of a missing or malformed `package.json` — that's normal during
 * `lando init` on a new project. The default artifact directory falls
 * back to `<projectRoot>/.lando`.
 */
export function readProjectMeta(projectRoot: string): ReadProjectMetaResult {
  if (!isAbsolute(projectRoot)) {
    return {
      ok: false,
      errors: [
        {
          code: CONFIG_ERROR_CODES.L_CONFIG_PROJECT_META,
          message: `projectRoot must be absolute, got ${JSON.stringify(projectRoot)}`,
          path: 'projectRoot',
        },
      ],
    };
  }

  let raw: string;
  try {
    raw = readFileSync(resolve(projectRoot, 'package.json'), 'utf8');
  } catch {
    return {
      ok: true,
      meta: { projectRoot, artifactDir: resolve(projectRoot, '.lando') },
    };
  }

  let parsed: unknown;
  try {
    parsed = JSON.parse(raw);
  } catch (cause) {
    return {
      ok: false,
      errors: [
        {
          code: CONFIG_ERROR_CODES.L_CONFIG_PROJECT_META,
          message: `failed to parse package.json: ${(cause as Error).message}`,
          path: 'package.json',
          cause,
        },
      ],
    };
  }

  const dir = readLandoDir(parsed);
  return {
    ok: true,
    meta: {
      projectRoot,
      artifactDir: dir === undefined ? resolve(projectRoot, '.lando') : resolve(projectRoot, dir),
    },
  };
}

/**
 * Walks upward from `cwd` looking for the nearest `package.json`. Stops
 * at the filesystem root. Returns the directory containing it, or
 * `undefined` if none was found.
 */
export function findProjectRoot(cwd: string): string | undefined {
  let current = resolve(cwd);
  while (true) {
    try {
      readFileSync(resolve(current, 'package.json'), 'utf8');
      return current;
    } catch {
      const parent = dirname(current);
      if (parent === current) return undefined;
      current = parent;
    }
  }
}

function readLandoDir(pkg: unknown): string | undefined {
  if (typeof pkg !== 'object' || pkg === null) return undefined;
  const lando = (pkg as Record<string, unknown>)['lando'];
  if (typeof lando !== 'object' || lando === null) return undefined;
  const dir = (lando as Record<string, unknown>)['dir'];
  return typeof dir === 'string' ? dir : undefined;
}
