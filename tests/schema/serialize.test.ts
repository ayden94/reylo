import { describe, expect, it } from 'vitest';
import {
  canonicalize,
  parseSchemaModule,
  printSchemaModule,
  SchemaParseError,
  validateSchema,
} from '#schema';
import { all } from '#test/fixtures/schemas/index';

describe('printSchemaModule / parseSchemaModule', () => {
  it('round-trips every fixture', () => {
    for (const [name, schema] of Object.entries(all)) {
      const printed = printSchemaModule(schema);
      const parsed = parseSchemaModule(printed);
      expect(parsed, `round-trip mismatch for ${name}`).toEqual(canonicalize(schema));
      // Round-trip must preserve validity.
      expect(validateSchema(parsed), `validation regressed for ${name}`).toEqual({ ok: true });
    }
  });

  it('emits byte-stable output for structurally equal inputs', () => {
    const a = printSchemaModule(all.simple);
    // Re-shuffle relations + model keys; canonicalize sorts them back.
    const shuffled = {
      ...all.simple,
      models: {
        Post: all.simple.models['Post']!,
        User: all.simple.models['User']!,
      },
    };
    const b = printSchemaModule(shuffled);
    expect(b).toEqual(a);
  });

  it('matches the snapshot for the polymorphic-association fixture', () => {
    expect(printSchemaModule(all.withAssociation)).toMatchSnapshot();
  });

  it('rejects modules without a default export', () => {
    expect(() => parseSchemaModule('export const x = 1;')).toThrow(SchemaParseError);
  });

  it('rejects literal expressions other than data', () => {
    const src = `export default { foo: bar } satisfies LandoSchema;`;
    expect(() => parseSchemaModule(src)).toThrow(SchemaParseError);
  });

  it('tolerates `as const` and `satisfies` on the default export', () => {
    const stripped = printSchemaModule(all.simple).replace(
      / satisfies LandoSchema;/,
      ' as const;',
    );
    const parsed = parseSchemaModule(stripped);
    expect(parsed).toEqual(canonicalize(all.simple));
  });
});
