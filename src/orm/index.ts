// Public surface of the `lando` package root. The ORM (Phases 8-10)
// will land its query API here; for now we re-export the schema-kit
// types and the config types so users can compose them in
// `lando.config.ts`:
//
//   import type { LandoConfig, LandoSchema } from 'lando';
//   import { pg } from 'lando/pg';
//
//   export default {
//     source: { kind: 'prisma', path: './prisma/schema.prisma' },
//     adapter: pg(),
//   } satisfies LandoConfig;

export type {
  Adapter,
  AdapterCapabilities,
  AssociationConfig,
  AssociationVariant,
  DefaultValue,
  Enum,
  ErrorCode,
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
  SchemaSource as LandoSchemaSource,
  UniqueConstraint,
  ValidationError,
  ValidationResult,
  VariantSpec,
} from '#schema';
export { ERROR_CODES, FORMAT_VERSION } from '#schema';

export type { LandoConfig, SchemaSource as LandoConfigSource } from '#config';
