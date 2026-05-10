// Phase 1: the in-memory schema model.
//
// `LandoSchema` is the single artifact the rest of the package agrees on:
// schema sources (Prisma, introspection) produce it, the ORM consumes it
// for type inference and SQL compilation, and it round-trips to disk as
// `.lando/schema.ts` — see `./serialize.ts` and the FORMAT.md sibling.
//
// Design rules (see whitepaper §"Generated artifacts"):
//   - Plain data only. No methods, classes, branded types, or non-literal
//     values. Anything that survives `JSON.stringify` should round-trip.
//   - Discriminated unions (`kind: ...`) over enums. Easier to walk in
//     consumers and TS-friendly when narrowing.
//   - Optional fields use `?:`, not `T | undefined`. With
//     `exactOptionalPropertyTypes` on, missing-vs-undefined is a real
//     distinction and we want missing.

/** Bumped on any breaking change to the artifact shape. */
export const FORMAT_VERSION = 1 as const;
export type FormatVersion = typeof FORMAT_VERSION;

export type LandoSchema = {
  formatVersion: FormatVersion;
  models: Record<string, Model>;
  enums: Record<string, Enum>;
  relations: Relation[];
  metadata: SchemaMetadata;
};

export type SchemaMetadata = {
  /** ISO-8601 timestamp of when the artifact was generated. */
  generatedAt: string;
  /** `lando@x.y.z` of the generator. */
  generatedBy: string;
  source: SchemaSource;
  /** SHA-256 over the canonicalized schema body, hex. */
  checksum: string;
};

export type SchemaSource =
  | { kind: 'prisma'; path: string }
  | { kind: 'introspection'; fingerprint: string };

// Models -----------------------------------------------------------------

export type Model = {
  name: string;
  tableName: string;
  /** Declaration order is semantically meaningful — preserved as given. */
  fields: Field[];
  primaryKey: PrimaryKey;
  uniques: UniqueConstraint[];
  indexes: Index[];
  polymorphism?: PolymorphismConfig;
};

export type Field = {
  name: string;
  /** Physical column name. Equal to `name` unless overridden by `@map`. */
  columnName: string;
  type: FieldType;
  nullable: boolean;
  default?: DefaultValue;
  attributes: FieldAttributes;
};

export type FieldType =
  | { kind: 'scalar'; scalar: ScalarType }
  | { kind: 'enum'; enum: string }
  | { kind: 'relation'; relation: string }
  | { kind: 'json' }
  | { kind: 'unknown'; native: string };

export type ScalarType =
  | 'string'
  | 'int'
  | 'bigint'
  | 'float'
  | 'decimal'
  | 'boolean'
  | 'date'
  | 'dateTime'
  | 'time'
  | 'bytes'
  | 'uuid'
  | 'json';

export type DefaultValue =
  | { kind: 'literal'; value: string | number | boolean | null }
  | { kind: 'expression'; sql: string }
  | { kind: 'autoincrement' }
  | { kind: 'now' }
  | { kind: 'uuid' };

export type FieldAttributes = {
  /** `GENERATED ALWAYS AS (...) STORED` in SQL. Read-only on insert. */
  generated?: boolean;
  /** `SERIAL` / `IDENTITY`. Caller may omit on insert. */
  identity?: boolean;
  /** Result of `@db.VarChar(N)` and friends. */
  maxLength?: number;
  precision?: { precision: number; scale: number };
  array?: boolean;
};

export type PrimaryKey = {
  /** Field names (not column names). */
  fields: string[];
};

export type UniqueConstraint = {
  name?: string;
  fields: string[];
};

export type Index = {
  name?: string;
  fields: string[];
  unique: boolean;
};

// Enums ------------------------------------------------------------------

export type Enum = {
  name: string;
  /** Declaration order is semantically meaningful (matches PG enum order). */
  values: string[];
};

// Relations --------------------------------------------------------------

/**
 * Relations are top-level. Each is referenced from two `Field`s (one on
 * each side of the relation) via `FieldType { kind: 'relation', relation:
 * <name> }`. Storing them once at the schema level keeps both sides
 * consistent — there's no way for the forward and reverse declarations to
 * drift.
 */
export type Relation = {
  /** Stable name; for Prisma sources, the value of `@relation("name")`. */
  name: string;
  kind: 'one-to-one' | 'one-to-many' | 'many-to-many';
  /** Model owning the FK. For many-to-many, the side with the link table. */
  from: string;
  to: string;
  /**
   * FK columns in `from`'s table. Empty for back-relation virtual fields
   * (the side without the FK).
   */
  fkColumns: string[];
  /** PK columns in `to`'s table referenced by `fkColumns`. */
  references: string[];
  onDelete?: ReferentialAction;
  onUpdate?: ReferentialAction;
  /** Field on `from` that points at `to`. */
  forwardName: string;
  /** Field on `to` that points at `from`. */
  reverseName: string;
};

export type ReferentialAction =
  | 'cascade'
  | 'restrict'
  | 'set-null'
  | 'set-default'
  | 'no-action';

// Polymorphism -----------------------------------------------------------

export type PolymorphismConfig = InheritanceConfig | AssociationConfig;

/**
 * Variant inheritance. v0.1 ships `single-table` only; the other strategies
 * are typed here so the format doesn't churn when v0.2 lands.
 */
export type InheritanceConfig = {
  kind: 'inheritance';
  /** Column whose value selects the variant. */
  discriminator: string;
  strategy: 'single-table' | 'joined' | 'class-table' | 'embedded-json';
  variants: Record<string, VariantSpec>;
};

export type VariantSpec = {
  /** Discriminator value that selects this variant. Defaults to the key. */
  value?: string;
  /** Field names (on the polymorphic root) that belong to this variant. */
  include?: string[];
  /** For `joined` / `class-table`: the variant table. Unused in v0.1. */
  table?: string;
};

/**
 * Polymorphic association — exclusive arc. The model carries one nullable
 * FK column per target type, plus a `CHECK (num_nonnulls(...) = 1)`
 * constraint enforced in the DB.
 */
export type AssociationConfig = {
  kind: 'association';
  /** Virtual TS field name presented as a typed union. */
  target: string;
  variants: Record<string, AssociationVariant>;
};

export type AssociationVariant = {
  /** Nullable FK column on the holding table. */
  column: string;
  /** Target model name. */
  references: string;
};
