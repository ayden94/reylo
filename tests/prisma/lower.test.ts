import { readFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import { dirname, resolve } from 'node:path';
import { describe, expect, it } from 'vitest';
import { canonicalize, validateSchema, ERROR_CODES } from '#schema';
import {
  applyPolymorphism,
  lowerPrisma,
  parsePrismaSource,
  PrismaLoweringError,
} from '#prisma';

const FIXTURES = resolve(dirname(fileURLToPath(import.meta.url)), '../fixtures/prisma');

const STAMPED = {
  generatedAt: '2026-01-01T00:00:00.000Z',
  generatedBy: 'lando@0.0.0-fixture',
} as const;

function lowerFixture(name: string): ReturnType<typeof lowerPrisma> {
  const source = readFileSync(resolve(FIXTURES, `${name}.prisma`), 'utf8');
  const parsed = parsePrismaSource(source);
  return lowerPrisma(parsed, STAMPED);
}

const FIXTURES_LIST = [
  'simple',
  'enums',
  'composite-key',
  'self-relation',
  'multiple-relations',
  'm2m',
] as const;

describe('lowerPrisma — fixtures', () => {
  for (const name of FIXTURES_LIST) {
    it(`${name}: lowers and validates`, () => {
      const schema = canonicalize(lowerFixture(name));
      expect(validateSchema(schema), `validation failed for ${name}`).toEqual({ ok: true });
      expect(schema).toMatchSnapshot();
    });
  }
});

describe('lowerPrisma — failure paths', () => {
  it('rejects a model with no primary key', () => {
    const src = `
model Thing {
  name String
}
`;
    expect(() => lowerPrisma(parsePrismaSource(src))).toThrow(PrismaLoweringError);
  });

  it('rejects a model with multiple field-level @id', () => {
    const src = `
model Thing {
  a Int @id
  b Int @id
}
`;
    expect(() => lowerPrisma(parsePrismaSource(src))).toThrow(PrismaLoweringError);
  });

  it('rejects ambiguous relations between the same two models', () => {
    const src = `
model User {
  id     Int    @id
  postsA Post[]
  postsB Post[]
}

model Post {
  id       Int  @id
  authorId Int
  author   User @relation(fields: [authorId], references: [id])
}
`;
    expect(() => lowerPrisma(parsePrismaSource(src))).toThrow(PrismaLoweringError);
  });

  it('parses a datasource and exposes its env-backed url', () => {
    const src = `
datasource db {
  provider = "postgresql"
  url      = env("DATABASE_URL")
}
`;
    const parsed = parsePrismaSource(src);
    expect(parsed.datasource).toEqual({
      name: 'db',
      provider: 'postgresql',
      url: { kind: 'env', name: 'DATABASE_URL' },
    });
  });
});

describe('applyPolymorphism', () => {
  it('attaches a single-table inheritance config and validates the merged schema', () => {
    const src = `
model Account {
  id          Int     @id
  kind        String
  displayName String?
  permissions Json?
}
`;
    const lowered = lowerPrisma(parsePrismaSource(src), STAMPED);
    const merged = applyPolymorphism(lowered, {
      Account: {
        kind: 'inheritance',
        discriminator: 'kind',
        strategy: 'single-table',
        variants: {
          user: { include: ['displayName'] },
          admin: { include: ['permissions'] },
        },
      },
    });
    if (!merged.ok) expect.fail(`unexpected errors: ${JSON.stringify(merged.errors)}`);
    expect(merged.schema.models['Account']?.polymorphism?.kind).toEqual('inheritance');
  });

  it('reports an error for an overlay referencing an unknown model', () => {
    const lowered = lowerPrisma(
      parsePrismaSource(`
model A {
  id Int @id
}
`),
      STAMPED,
    );
    const merged = applyPolymorphism(lowered, {
      Ghost: {
        kind: 'inheritance',
        discriminator: 'kind',
        strategy: 'single-table',
        variants: { x: {} },
      },
    });
    if (merged.ok) expect.fail('expected overlay to fail');
    expect(merged.errors.some((e) => e.code === ERROR_CODES.L_SCHEMA_UNKNOWN_MODEL)).toBe(true);
  });
});
