// Prisma AST → LandoSchema lowering. Pure function over the parsed AST;
// no I/O, no globals. Pairs with `parser.ts` (the AST-producing side) and
// `polymorphism.ts` (the post-lowering overlay).
//
// The goal here is high-fidelity coverage of the *common* Prisma surface
// (90% of real schemas, per the v0.1 wedge). Less-common shapes
// (composite types, exotic native types, runtime DB functions in
// defaults) are surfaced as `unknown` field types or `expression`
// defaults, which downstream code can either accept verbatim or warn on.

import type { Schema as PrismaAst } from '@mrleebo/prisma-ast';
import type {
  DefaultValue,
  Enum,
  Field,
  FieldAttributes,
  FieldType,
  Index,
  LandoSchema,
  Model,
  ReferentialAction,
  Relation,
  ScalarType,
  UniqueConstraint,
} from '#schema';
import { FORMAT_VERSION } from '#schema';
import type { ParsedPrisma } from './parser.ts';

export type LowerOptions = {
  /** Stamped into `metadata.generatedBy`. */
  generatedBy?: string;
  /** Stamped into `metadata.generatedAt`. Defaults to `new Date().toISOString()`. */
  generatedAt?: string;
};

export class PrismaLoweringError extends Error {
  override readonly name = 'PrismaLoweringError';
  constructor(message: string) {
    super(message);
  }
}

/**
 * Lowers a parsed Prisma AST into a `LandoSchema`. Polymorphism is *not*
 * applied here — that's a separate overlay (`./polymorphism.ts`) so that
 * the Prisma-only path stays pure and deterministic.
 *
 * The returned schema has empty `metadata.checksum`; the caller (the CLI)
 * is responsible for stamping the checksum after canonicalization.
 */
export function lowerPrisma(parsed: ParsedPrisma, opts: LowerOptions = {}): LandoSchema {
  const ast = parsed.ast;
  const enums: Record<string, Enum> = {};
  const enumNames = new Set<string>();
  const modelNames = new Set<string>();

  for (const node of ast.list) {
    if (node.type === 'enum') enumNames.add(node.name);
    if (node.type === 'model') modelNames.add(node.name);
  }

  for (const node of ast.list) {
    if (node.type !== 'enum') continue;
    enums[node.name] = lowerEnum(node);
  }

  const models: Record<string, Model> = {};
  // Relation-field information collected per-model; the second pass
  // pairs forward + back relation fields into top-level Relations and
  // back-fills the field-level relation name (which depends on the
  // pairing).
  const relationFields: PendingRelationField[] = [];

  for (const node of ast.list) {
    if (node.type !== 'model') continue;
    const lowered = lowerModel(node, enumNames, modelNames, relationFields);
    models[node.name] = lowered;
  }

  const relations = pairRelations(relationFields, models);
  for (const r of relations) {
    patchRelationFieldName(models, r.from, r.forwardName, r.name);
    patchRelationFieldName(models, r.to, r.reverseName, r.name);
  }

  const source =
    parsed.path !== undefined
      ? ({ kind: 'prisma', path: parsed.path } as const)
      : ({ kind: 'prisma', path: '<inline>' } as const);

  return {
    formatVersion: FORMAT_VERSION,
    models,
    enums,
    relations,
    metadata: {
      generatedAt: opts.generatedAt ?? new Date().toISOString(),
      generatedBy: opts.generatedBy ?? 'lando@0.0.0',
      source,
      checksum: '',
    },
  };
}

// Enums -----------------------------------------------------------------

function lowerEnum(node: PrismaEnumNode): Enum {
  return {
    name: node.name,
    values: node.enumerators
      .filter((e): e is { type: 'enumerator'; name: string } => e.type === 'enumerator')
      .map((e) => e.name),
  };
}

// Models ----------------------------------------------------------------

function lowerModel(
  node: PrismaModelNode,
  enumNames: ReadonlySet<string>,
  modelNames: ReadonlySet<string>,
  relationFields: PendingRelationField[],
): Model {
  const fields: Field[] = [];
  const blockUniques: UniqueConstraint[] = [];
  const blockIndexes: Index[] = [];
  let tableName = node.name;
  const fieldUniques: UniqueConstraint[] = [];
  let primaryKeyFields: string[] | undefined;

  for (const prop of node.properties) {
    if (prop.type === 'field') {
      const lowered = lowerField(prop, node.name, enumNames, modelNames, relationFields);
      if (lowered === undefined) continue;
      fields.push(lowered.field);
      if (lowered.isId) {
        if (primaryKeyFields !== undefined) {
          throw new PrismaLoweringError(
            `model "${node.name}" has multiple field-level @id; use @@id for composite keys`,
          );
        }
        primaryKeyFields = [lowered.field.name];
      }
      if (lowered.isUnique) {
        fieldUniques.push({ fields: [lowered.field.name] });
      }
      continue;
    }
    if (prop.type === 'attribute' && prop.kind === 'object') {
      const blockAttr = prop as unknown as PrismaAttribute;
      switch (blockAttr.name) {
        case 'map': {
          const v = readFirstStringArg(blockAttr);
          if (v !== undefined) tableName = v;
          break;
        }
        case 'id': {
          const cols = readFirstArrayArg(blockAttr);
          if (cols !== undefined) primaryKeyFields = cols;
          break;
        }
        case 'unique': {
          const cols = readFirstArrayArg(blockAttr);
          if (cols !== undefined) blockUniques.push({ fields: cols });
          break;
        }
        case 'index': {
          const cols = readFirstArrayArg(blockAttr);
          if (cols !== undefined) blockIndexes.push({ fields: cols, unique: false });
          break;
        }
        // Other block attributes (`@@ignore`, `@@schema`, etc.) are
        // ignored in v0.1 — they don't affect the schema model shape.
      }
    }
  }

  if (primaryKeyFields === undefined) {
    throw new PrismaLoweringError(`model "${node.name}" has no primary key (@id or @@id)`);
  }

  return {
    name: node.name,
    tableName,
    fields,
    primaryKey: { fields: primaryKeyFields },
    uniques: [...fieldUniques, ...blockUniques],
    indexes: blockIndexes,
  };
}

// Fields ----------------------------------------------------------------

type LoweredField = { field: Field; isId: boolean; isUnique: boolean };

function lowerField(
  prop: PrismaFieldNode,
  modelName: string,
  enumNames: ReadonlySet<string>,
  modelNames: ReadonlySet<string>,
  relationFields: PendingRelationField[],
): LoweredField | undefined {
  const fieldType = prop.fieldType;
  if (typeof fieldType !== 'string') {
    // Composite types (Prisma 5+) appear as objects here. Treat the
    // field as JSON for v0.1 and move on.
    const field: Field = {
      name: prop.name,
      columnName: prop.name,
      type: { kind: 'json' },
      nullable: prop.optional ?? false,
      attributes: { array: prop.array ?? false },
    };
    return { field, isId: false, isUnique: false };
  }

  let isId = false;
  let isUnique = false;
  let columnName = prop.name;
  let defaultValue: DefaultValue | undefined;
  const attrs: FieldAttributes = {};
  let relation: PrismaRelationAttribute | undefined;
  /** Native-type override; takes precedence over the base Prisma scalar. */
  let scalarOverride: ScalarType | undefined;

  for (const a of prop.attributes ?? []) {
    if (a.kind !== 'field') continue;
    if (a.group === 'db') {
      // Native types like @db.VarChar(255), @db.Uuid, @db.Text, ...
      const override = applyNativeType(a, attrs);
      if (override !== undefined) scalarOverride = override;
      continue;
    }
    switch (a.name) {
      case 'id':
        isId = true;
        break;
      case 'unique':
        isUnique = true;
        break;
      case 'map': {
        const v = readFirstStringArg(a);
        if (v !== undefined) columnName = v;
        break;
      }
      case 'default':
        defaultValue = readDefault(a);
        break;
      case 'relation':
        relation = readRelationAttribute(a);
        break;
      case 'updatedAt':
        // Prisma's @updatedAt -- application-level concern, not a schema concern.
        break;
      default:
        // Unknown attribute. Skip silently; it's not load-bearing for v0.1.
        break;
    }
  }

  // Decide the field's type. Order matters: native type may have set the
  // scalar (e.g. @db.Uuid → uuid) and we don't want to clobber it.
  const isRelation = modelNames.has(fieldType);
  if (isRelation) {
    // Side resolution: the *forward* (FK-owning) side is the one whose
    // @relation carries `fields:`. Everything else is the reverse side.
    // m2m implicit (both arrays, no @relation) ends up reverse on both
    // sides; pairing handles that case explicitly.
    const ownsFk = (relation?.fields?.length ?? 0) > 0;
    const pending: PendingRelationField = {
      modelName,
      fieldName: prop.name,
      target: fieldType,
      side: ownsFk ? 'forward' : 'reverse',
      array: prop.array ?? false,
      nullable: prop.optional ?? false,
    };
    if (relation !== undefined) pending.relation = relation;
    relationFields.push(pending);
    const field: Field = {
      name: prop.name,
      columnName: '',
      type: { kind: 'relation', relation: '' /* filled in pairing */ },
      nullable: prop.optional ?? false,
      attributes: { array: prop.array ?? false },
    };
    return { field, isId, isUnique };
  }

  const type: FieldType = enumNames.has(fieldType)
    ? { kind: 'enum', enum: fieldType }
    : scalarOverride !== undefined
      ? { kind: 'scalar', scalar: scalarOverride }
      : prismaScalarToFieldType(fieldType, attrs);

  if (prop.array) attrs.array = true;

  const field: Field = {
    name: prop.name,
    columnName,
    type,
    nullable: prop.optional ?? false,
    attributes: attrs,
  };
  if (defaultValue !== undefined) field.default = defaultValue;
  return { field, isId, isUnique };
}

function prismaScalarToFieldType(name: string, attrs: FieldAttributes): FieldType {
  // `attrs` may already carry refinements set by the native-type pass —
  // for instance, `@db.Uuid` set `attrs.maxLength: 36` (we don't, but
  // similar shape). Refinements like `maxLength` ride on the attributes
  // object itself; the scalar kind here is the high-level Prisma type.
  void attrs;
  switch (name) {
    case 'String':
      return { kind: 'scalar', scalar: 'string' };
    case 'Int':
      return { kind: 'scalar', scalar: 'int' };
    case 'BigInt':
      return { kind: 'scalar', scalar: 'bigint' };
    case 'Float':
      return { kind: 'scalar', scalar: 'float' };
    case 'Decimal':
      return { kind: 'scalar', scalar: 'decimal' };
    case 'Boolean':
      return { kind: 'scalar', scalar: 'boolean' };
    case 'DateTime':
      return { kind: 'scalar', scalar: 'dateTime' };
    case 'Bytes':
      return { kind: 'scalar', scalar: 'bytes' };
    case 'Json':
      return { kind: 'scalar', scalar: 'json' };
    default:
      return { kind: 'unknown', native: name };
  }
}

function applyNativeType(a: PrismaAttribute, attrs: FieldAttributes): ScalarType | undefined {
  // `@db.<Name>(args...)`. Common ones change the inferred scalar; less
  // common ones just attach attributes. Anything we don't recognize is
  // a no-op — the base Prisma type still applies.
  switch (a.name) {
    case 'VarChar':
    case 'Char': {
      const n = readFirstNumberArg(a);
      if (n !== undefined) attrs.maxLength = n;
      return 'string';
    }
    case 'Text':
      return 'string';
    case 'Uuid':
      // Override: @db.Uuid on String → uuid scalar.
      return 'uuid';
    case 'Date':
      return 'date';
    case 'Time':
      return 'time';
    case 'Timestamp':
    case 'Timestamptz':
      return 'dateTime';
    case 'Decimal': {
      const args = a.args ?? [];
      const p = args[0]?.value;
      const s = args[1]?.value;
      if (typeof p === 'string' && typeof s === 'string') {
        const pn = Number(p);
        const sn = Number(s);
        if (Number.isFinite(pn) && Number.isFinite(sn)) {
          attrs.precision = { precision: pn, scale: sn };
        }
      }
      return 'decimal';
    }
    default:
      return undefined;
  }
}

function readDefault(a: PrismaAttribute): DefaultValue | undefined {
  const arg = a.args?.[0];
  if (arg === undefined) return undefined;
  const v = arg.value;
  if (v === undefined || v === null) return undefined;
  if (typeof v === 'string') {
    // Prisma stringifies literals with their quotes intact: '"hello"'.
    // Numeric defaults appear as the unquoted text: '0'.
    if (v.length >= 2 && v.startsWith('"') && v.endsWith('"')) {
      return { kind: 'literal', value: v.slice(1, -1) };
    }
    if (v === 'true') return { kind: 'literal', value: true };
    if (v === 'false') return { kind: 'literal', value: false };
    if (v === 'null') return { kind: 'literal', value: null };
    const n = Number(v);
    if (Number.isFinite(n) && /^-?\d/.test(v)) return { kind: 'literal', value: n };
    // Otherwise it's an enum value reference (e.g. `@default(user)` on a
    // Role-typed field). Store as a literal string; consumers can
    // disambiguate using the field type.
    return { kind: 'literal', value: v };
  }
  if (typeof v === 'object' && 'type' in v && v.type === 'function') {
    const fn = v as unknown as { name: string; params?: unknown[] };
    if (fn.name === 'autoincrement') return { kind: 'autoincrement' };
    if (fn.name === 'now') return { kind: 'now' };
    if (fn.name === 'uuid' || fn.name === 'cuid') return { kind: 'uuid' };
    if (fn.name === 'dbgenerated') {
      const inner = (fn.params?.[0] ?? '') as string;
      const trimmed = inner.length >= 2 && inner.startsWith('"') && inner.endsWith('"')
        ? inner.slice(1, -1)
        : inner;
      return { kind: 'expression', sql: trimmed };
    }
  }
  return undefined;
}

// Relations -------------------------------------------------------------

type PrismaRelationAttribute = {
  name?: string;
  fields?: string[];
  references?: string[];
  onDelete?: ReferentialAction;
  onUpdate?: ReferentialAction;
};

function readRelationAttribute(a: PrismaAttribute): PrismaRelationAttribute {
  const out: PrismaRelationAttribute = {};
  for (const arg of a.args ?? []) {
    const v = arg.value;
    if (typeof v === 'object' && v !== null && 'type' in v && v.type === 'keyValue') {
      const kv = v as unknown as { key: string; value: unknown };
      switch (kv.key) {
        case 'fields': {
          const arr = readArrayArgValue(kv.value);
          if (arr !== undefined) out.fields = arr;
          break;
        }
        case 'references': {
          const arr = readArrayArgValue(kv.value);
          if (arr !== undefined) out.references = arr;
          break;
        }
        case 'onDelete': {
          const action = mapReferentialAction(kv.value);
          if (action !== undefined) out.onDelete = action;
          break;
        }
        case 'onUpdate': {
          const action = mapReferentialAction(kv.value);
          if (action !== undefined) out.onUpdate = action;
          break;
        }
        case 'name': {
          const name = readMaybeQuotedString(kv.value);
          if (name !== undefined) out.name = name;
          break;
        }
      }
    } else if (typeof v === 'string') {
      const stripped = readMaybeQuotedString(v);
      if (stripped !== undefined) out.name = stripped;
    }
  }
  return out;
}

type PendingRelationField = {
  modelName: string;
  fieldName: string;
  target: string;
  side: 'forward' | 'reverse';
  array: boolean;
  nullable: boolean;
  relation?: PrismaRelationAttribute;
};

function pairRelations(
  pending: PendingRelationField[],
  models: Record<string, Model>,
): Relation[] {
  const out: Relation[] = [];
  const used = new Set<number>();

  for (let i = 0; i < pending.length; i++) {
    if (used.has(i)) continue;
    const left = pending[i];
    if (left === undefined) continue;

    // Match candidates: same two models, opposite sides — except for
    // implicit m2m (both arrays, both reverse, no @relation), which we
    // allow to pair same-side.
    const candidates: number[] = [];
    for (let j = 0; j < pending.length; j++) {
      if (j === i || used.has(j)) continue;
      const right = pending[j];
      if (right === undefined) continue;
      if (right.modelName !== left.target) continue;
      if (right.target !== left.modelName) continue;

      const isImplicitM2M =
        left.array &&
        right.array &&
        left.relation?.fields === undefined &&
        right.relation?.fields === undefined;
      if (right.side === left.side && !isImplicitM2M) continue;

      // Relation-name agreement: when either side declares a name, both
      // must agree on it.
      const leftName = left.relation?.name;
      const rightName = right.relation?.name;
      if (leftName !== undefined || rightName !== undefined) {
        if (leftName === rightName) candidates.push(j);
      } else {
        candidates.push(j);
      }
    }
    if (candidates.length === 0) {
      throw new PrismaLoweringError(
        `relation field "${left.modelName}.${left.fieldName}" → "${left.target}" has no matching back-relation`,
      );
    }
    if (candidates.length > 1) {
      throw new PrismaLoweringError(
        `relation field "${left.modelName}.${left.fieldName}" → "${left.target}" is ambiguous; add @relation("name") to disambiguate`,
      );
    }
    const partnerIndex = candidates[0]!;
    const partner = pending[partnerIndex]!;
    used.add(i);
    used.add(partnerIndex);

    // For implicit m2m we still need to pick *some* "from" — the
    // alphabetically-earlier model is the canonical choice. (Prisma uses
    // an auto-generated link table whose columns are based on this
    // ordering; downstream consumers don't rely on it but the
    // canonicalizer will still emit a stable result.)
    const isImplicitM2M = left.array && partner.array && left.side === partner.side;
    const forward = isImplicitM2M
      ? left.modelName <= partner.modelName
        ? left
        : partner
      : left.side === 'forward'
        ? left
        : partner;
    const reverse = forward === left ? partner : left;

    const relName =
      forward.relation?.name ??
      reverse.relation?.name ??
      defaultRelationName(forward.modelName, reverse.modelName);

    const kind: Relation['kind'] = isImplicitM2M
      ? 'many-to-many'
      : forward.array || reverse.array
        ? 'one-to-many'
        : 'one-to-one';

    // Translate Prisma field names (in `@relation(fields: [...])`) to
    // physical column names — the schema model's `fkColumns` /
    // `references` are column names, not field names.
    const fkColumns = (forward.relation?.fields ?? []).map((fname) =>
      fieldColumnName(models[forward.modelName], fname),
    );
    const references = (forward.relation?.references ?? []).map((fname) =>
      fieldColumnName(models[reverse.modelName], fname),
    );

    const rel: Relation = {
      name: relName,
      kind,
      from: forward.modelName,
      to: reverse.modelName,
      fkColumns,
      references,
      forwardName: forward.fieldName,
      reverseName: reverse.fieldName,
    };
    if (forward.relation?.onDelete) rel.onDelete = forward.relation.onDelete;
    if (forward.relation?.onUpdate) rel.onUpdate = forward.relation.onUpdate;
    out.push(rel);
  }

  return out;
}

function fieldColumnName(model: Model | undefined, fieldName: string): string {
  if (model === undefined) return fieldName;
  const f = model.fields.find((ff) => ff.name === fieldName);
  return f?.columnName ?? fieldName;
}

function patchRelationFieldName(
  models: Record<string, Model>,
  modelName: string,
  fieldName: string,
  relationName: string,
): void {
  const model = models[modelName];
  if (model === undefined) return;
  const field = model.fields.find((f) => f.name === fieldName);
  if (field === undefined) return;
  if (field.type.kind === 'relation') {
    field.type = { kind: 'relation', relation: relationName };
  }
}

function defaultRelationName(from: string, to: string): string {
  // Prisma's default relation name for unnamed relations is the
  // alphabetised pair of model names joined by "To".
  return [from, to].sort().join('To');
}

function mapReferentialAction(value: unknown): ReferentialAction | undefined {
  if (typeof value !== 'string') return undefined;
  switch (value) {
    case 'Cascade':
      return 'cascade';
    case 'Restrict':
      return 'restrict';
    case 'SetNull':
      return 'set-null';
    case 'SetDefault':
      return 'set-default';
    case 'NoAction':
      return 'no-action';
    default:
      return undefined;
  }
}

// AST narrowing ---------------------------------------------------------
//
// `@mrleebo/prisma-ast`'s exported types are looser than the actual AST
// shape (most things are typed as `string | unknown`). The narrow types
// below describe just the slice we care about.

type PrismaModelNode = Extract<PrismaAst['list'][number], { type: 'model' }>;
type PrismaEnumNode = Extract<PrismaAst['list'][number], { type: 'enum' }>;
type PrismaModelProperty = PrismaModelNode['properties'][number];
type PrismaFieldNode = Extract<PrismaModelProperty, { type: 'field' }>;
type PrismaAttribute = {
  type: 'attribute';
  name: string;
  /** AST also models 'view' and 'type' for newer Prisma features; we treat them as no-ops here. */
  kind?: 'field' | 'object' | 'view' | 'type';
  group?: string;
  args?: ReadonlyArray<{ type: string; value?: unknown }>;
};

function readFirstStringArg(a: PrismaAttribute): string | undefined {
  const v = a.args?.[0]?.value;
  return readMaybeQuotedString(v);
}

function readFirstNumberArg(a: PrismaAttribute): number | undefined {
  const v = a.args?.[0]?.value;
  if (typeof v !== 'string') return undefined;
  const n = Number(v);
  return Number.isFinite(n) ? n : undefined;
}

function readFirstArrayArg(a: PrismaAttribute): string[] | undefined {
  const v = a.args?.[0]?.value;
  return readArrayArgValue(v);
}

function readArrayArgValue(v: unknown): string[] | undefined {
  if (typeof v === 'object' && v !== null && 'type' in v && v.type === 'array') {
    const arr = v as { args?: unknown[] };
    return (arr.args ?? []).map((item) => {
      if (typeof item === 'string') return readMaybeQuotedString(item) ?? item;
      throw new PrismaLoweringError('non-string element inside array argument');
    });
  }
  return undefined;
}

function readMaybeQuotedString(v: unknown): string | undefined {
  if (typeof v !== 'string') return undefined;
  if (v.length >= 2 && v.startsWith('"') && v.endsWith('"')) return v.slice(1, -1);
  return v;
}
