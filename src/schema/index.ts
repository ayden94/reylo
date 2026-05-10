// Public surface of the schema kit. Stable from v0.1; additions are
// non-breaking, removals require a major version bump (and a documented
// migration in FORMAT.md).

export type {
  AssociationConfig,
  AssociationVariant,
  DefaultValue,
  Enum,
  Field,
  FieldAttributes,
  FieldType,
  FormatVersion,
  Index,
  InheritanceConfig,
  LandoSchema,
  Model,
  PolymorphismConfig,
  PrimaryKey,
  ReferentialAction,
  Relation,
  ScalarType,
  SchemaMetadata,
  SchemaSource,
  UniqueConstraint,
  VariantSpec,
} from './types.ts';
export { FORMAT_VERSION } from './types.ts';

export type { ErrorCode, ValidationError, ValidationResult } from './errors.ts';
export { ERROR_CODES, SchemaParseError } from './errors.ts';

export { validateSchema } from './validate.ts';
export { canonicalize } from './canonicalize.ts';
export { parseSchemaModule, printSchemaModule } from './serialize.ts';
export {
  forEachField,
  forEachRelation,
  getEnum,
  getModel,
  getPolymorphism,
  getRelation,
} from './walk.ts';
