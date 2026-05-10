import { dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { describe, expect, it } from 'vitest';
import {
  CONFIG_ERROR_CODES,
  findConfig,
  findProjectRoot,
  loadConfig,
  readProjectMeta,
  validateConfig,
} from '#config';
import { pg } from '#pg';

const FIXTURES = resolve(dirname(fileURLToPath(import.meta.url)), '../fixtures/configs');

describe('findConfig', () => {
  it('finds lando.config.ts in the immediate directory', () => {
    expect(findConfig(resolve(FIXTURES, 'valid-prisma'))).toEqual(
      resolve(FIXTURES, 'valid-prisma', 'lando.config.ts'),
    );
  });

  it('returns undefined when no config exists at or above', () => {
    // /tmp doesn't have a lando.config.ts in any ancestor.
    expect(findConfig('/tmp')).toBeUndefined();
  });
});

describe('readProjectMeta', () => {
  it('honors package.json#lando.dir', () => {
    const result = readProjectMeta(resolve(FIXTURES, 'with-lando-dir'));
    if (!result.ok) expect.fail(JSON.stringify(result.errors));
    expect(result.meta.artifactDir).toEqual(
      resolve(FIXTURES, 'with-lando-dir', 'generated'),
    );
  });

  it('falls back to <root>/.lando when no package.json is present', () => {
    const result = readProjectMeta(resolve(FIXTURES, 'valid-byo'));
    if (!result.ok) expect.fail(JSON.stringify(result.errors));
    expect(result.meta.artifactDir).toEqual(resolve(FIXTURES, 'valid-byo', '.lando'));
  });
});

describe('findProjectRoot', () => {
  it('walks up to find the nearest package.json', () => {
    const start = resolve(FIXTURES, 'with-lando-dir');
    expect(findProjectRoot(start)).toEqual(start);
  });
});

describe('loadConfig', () => {
  it('loads the prisma fixture and returns a validated config', async () => {
    const result = await loadConfig({ cwd: resolve(FIXTURES, 'valid-prisma') });
    if (!result.ok) expect.fail(JSON.stringify(result.errors));
    expect(result.config.source).toEqual({
      kind: 'prisma',
      path: './prisma/schema.prisma',
    });
    expect(result.config.adapter.kind).toEqual('pg');
    expect(result.config.polymorphism?.['Account']?.kind).toEqual('inheritance');
    // The fixture has no package.json of its own, so findProjectRoot
    // walks up to the lando project root. We don't assert the path —
    // the artifactDir behavior is covered by the readProjectMeta tests.
    expect(result.meta.artifactDir).toMatch(/\.lando|generated/);
  });

  it('loads the BYO-DB fixture and preserves introspectAt + validate', async () => {
    const result = await loadConfig({ cwd: resolve(FIXTURES, 'valid-byo') });
    if (!result.ok) expect.fail(JSON.stringify(result.errors));
    expect(result.config.introspectAt).toEqual('build');
    expect(result.config.validate).toEqual({ input: 'off' });
  });

  it('reports a structured error when the file is missing', async () => {
    const result = await loadConfig({ cwd: '/tmp' });
    expect(result.ok).toBe(false);
    if (result.ok) return;
    expect(result.errors[0]?.code).toEqual(CONFIG_ERROR_CODES.L_CONFIG_NOT_FOUND);
  });

  it('reports a structured error when the default export is missing', async () => {
    const result = await loadConfig({ cwd: resolve(FIXTURES, 'no-default-export') });
    expect(result.ok).toBe(false);
    if (result.ok) return;
    expect(result.errors[0]?.code).toEqual(CONFIG_ERROR_CODES.L_CONFIG_NO_DEFAULT_EXPORT);
  });

  it('reports a structured error when source.kind is invalid', async () => {
    const result = await loadConfig({ cwd: resolve(FIXTURES, 'invalid-source') });
    expect(result.ok).toBe(false);
    if (result.ok) return;
    expect(result.errors[0]?.code).toEqual(CONFIG_ERROR_CODES.L_CONFIG_INVALID_SOURCE);
  });
});

describe('validateConfig', () => {
  it('rejects a non-object config', () => {
    const result = validateConfig(42);
    if (result.ok) expect.fail('expected validation to fail');
    expect(result.errors[0]?.code).toEqual(CONFIG_ERROR_CODES.L_CONFIG_INVALID_SHAPE);
  });

  it('accepts a directly-constructed valid config', () => {
    const result = validateConfig({
      source: { kind: 'prisma', path: './schema.prisma' },
      adapter: pg(),
    });
    if (!result.ok) expect.fail(JSON.stringify(result.errors));
    expect(result.config.adapter.kind).toEqual('pg');
  });
});

describe('pg adapter', () => {
  it('exposes the four PG capability flags', () => {
    expect(pg().capabilities).toEqual({
      checkConstraints: true,
      expressionIndexes: true,
      lateral: true,
      skipLocked: true,
    });
  });
});
