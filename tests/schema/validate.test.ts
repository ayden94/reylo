import { describe, expect, it } from 'vitest';
import {
  ERROR_CODES,
  FORMAT_VERSION,
  type LandoSchema,
  validateSchema,
} from '#schema';
import { simple, withInheritance, withAssociation } from '#test/fixtures/schemas/index';

// One assertion per error code. Each fixture is a minimal mutation of a
// passing schema, so the test surface stays small even as new codes get
// added.
function assertError(s: LandoSchema, code: string, pathPrefix: string): void {
  const result = validateSchema(s);
  if (result.ok) {
    expect.fail(`expected validation error ${code}, got ok`);
  }
  const match = result.errors.find((e) => e.code === code && e.path.startsWith(pathPrefix));
  if (match === undefined) {
    expect.fail(
      `expected error ${code} under "${pathPrefix}". Got:\n${result.errors
        .map((e) => `  - ${e.code} @ ${e.path}: ${e.message}`)
        .join('\n')}`,
    );
  }
}

describe('validateSchema', () => {
  it('accepts valid fixtures', () => {
    expect(validateSchema(simple)).toEqual({ ok: true });
    expect(validateSchema(withInheritance)).toEqual({ ok: true });
    expect(validateSchema(withAssociation)).toEqual({ ok: true });
  });

  it('flags format version mismatch', () => {
    assertError(
      { ...simple, formatVersion: 999 as unknown as typeof FORMAT_VERSION },
      ERROR_CODES.L_SCHEMA_FORMAT_VERSION,
      'formatVersion',
    );
  });

  it('flags duplicate field names within a model', () => {
    const broken: LandoSchema = structuredClone(simple);
    const user = broken.models['User']!;
    user.fields.push({ ...user.fields[0]! });
    assertError(broken, ERROR_CODES.L_SCHEMA_DUPLICATE_FIELD, 'models.User.fields');
  });

  it('flags primary keys that reference unknown fields', () => {
    const broken: LandoSchema = structuredClone(simple);
    broken.models['User']!.primaryKey = { fields: ['nope'] };
    assertError(broken, ERROR_CODES.L_SCHEMA_UNKNOWN_FIELD, 'models.User.primaryKey');
  });

  it('flags an empty primary key', () => {
    const broken: LandoSchema = structuredClone(simple);
    broken.models['User']!.primaryKey = { fields: [] };
    assertError(broken, ERROR_CODES.L_SCHEMA_PRIMARY_KEY_INVALID, 'models.User.primaryKey');
  });

  it('flags relations that reference unknown models', () => {
    const broken: LandoSchema = structuredClone(simple);
    broken.relations[0]!.to = 'Ghost';
    assertError(broken, ERROR_CODES.L_SCHEMA_UNKNOWN_MODEL, 'relations[0].to');
  });

  it('flags fields whose enum reference is unknown', () => {
    const broken: LandoSchema = structuredClone(simple);
    broken.models['User']!.fields[1]!.type = { kind: 'enum', enum: 'Ghost' };
    assertError(broken, ERROR_CODES.L_SCHEMA_UNKNOWN_ENUM, 'models.User.fields[1]');
  });

  it('flags relation fields whose name disagrees with the relation', () => {
    const broken: LandoSchema = structuredClone(simple);
    // Rename `posts` field on User but keep relation.reverseName === 'posts'.
    broken.models['User']!.fields[2]!.name = 'wrong';
    assertError(broken, ERROR_CODES.L_SCHEMA_RELATION_MISMATCH, 'models.User.fields[2]');
  });

  it('flags inheritance polymorphism with an unknown discriminator column', () => {
    const broken: LandoSchema = structuredClone(withInheritance);
    broken.models['Account']!.polymorphism = {
      kind: 'inheritance',
      discriminator: 'nope',
      strategy: 'single-table',
      variants: { user: {} },
    };
    assertError(
      broken,
      ERROR_CODES.L_SCHEMA_INHERITANCE_DISCRIMINATOR_MISSING,
      'models.Account.polymorphism',
    );
  });

  it('flags inheritance variants whose include references unknown fields', () => {
    const broken: LandoSchema = structuredClone(withInheritance);
    const poly = broken.models['Account']!.polymorphism!;
    if (poly.kind !== 'inheritance') throw new Error('fixture mismatch');
    poly.variants = { user: { include: ['ghostField'] } };
    assertError(
      broken,
      ERROR_CODES.L_SCHEMA_INHERITANCE_VARIANT_INVALID,
      'models.Account.polymorphism',
    );
  });

  it('flags association polymorphism with fewer than two variants', () => {
    const broken: LandoSchema = structuredClone(withAssociation);
    const poly = broken.models['Comment']!.polymorphism!;
    if (poly.kind !== 'association') throw new Error('fixture mismatch');
    poly.variants = { post: poly.variants['post']! };
    assertError(
      broken,
      ERROR_CODES.L_SCHEMA_ASSOCIATION_TOO_FEW_VARIANTS,
      'models.Comment.polymorphism',
    );
  });

  it('flags association variants whose column is non-nullable', () => {
    const broken: LandoSchema = structuredClone(withAssociation);
    broken.models['Comment']!.fields.find((f) => f.columnName === 'post_id')!.nullable = false;
    assertError(
      broken,
      ERROR_CODES.L_SCHEMA_ASSOCIATION_VARIANT_INVALID,
      'models.Comment.polymorphism',
    );
  });

  it('flags association variants that reference unknown models', () => {
    const broken: LandoSchema = structuredClone(withAssociation);
    const poly = broken.models['Comment']!.polymorphism!;
    if (poly.kind !== 'association') throw new Error('fixture mismatch');
    poly.variants['post']!.references = 'Ghost';
    assertError(broken, ERROR_CODES.L_SCHEMA_UNKNOWN_MODEL, 'models.Comment.polymorphism');
  });
});
