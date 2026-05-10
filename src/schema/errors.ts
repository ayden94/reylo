// Stable error codes for schema-model validation and parsing. New codes
// only ever get *added*; never re-numbered. The set is small on purpose
// — generic "kind" + a structured `path` carry the specifics.

export const ERROR_CODES = {
  L_SCHEMA_FORMAT_VERSION: 'L_SCHEMA_001',
  L_SCHEMA_DUPLICATE_FIELD: 'L_SCHEMA_002',
  L_SCHEMA_UNKNOWN_FIELD: 'L_SCHEMA_003',
  L_SCHEMA_UNKNOWN_MODEL: 'L_SCHEMA_004',
  L_SCHEMA_UNKNOWN_ENUM: 'L_SCHEMA_005',
  L_SCHEMA_RELATION_MISMATCH: 'L_SCHEMA_006',
  L_SCHEMA_INHERITANCE_DISCRIMINATOR_MISSING: 'L_SCHEMA_007',
  L_SCHEMA_INHERITANCE_VARIANT_INVALID: 'L_SCHEMA_008',
  L_SCHEMA_ASSOCIATION_TOO_FEW_VARIANTS: 'L_SCHEMA_009',
  L_SCHEMA_ASSOCIATION_VARIANT_INVALID: 'L_SCHEMA_010',
  L_SCHEMA_PRIMARY_KEY_INVALID: 'L_SCHEMA_011',
  L_SCHEMA_PARSE: 'L_SCHEMA_100',
} as const;

export type ErrorCode = (typeof ERROR_CODES)[keyof typeof ERROR_CODES];

export type ValidationError = {
  code: ErrorCode;
  message: string;
  /** Dotted JSON path to the offending node, e.g. `models.User.fields[2]`. */
  path: string;
};

export type ValidationResult = { ok: true } | { ok: false; errors: ValidationError[] };

/**
 * Thrown by `parseSchemaModule` when the input isn't a recognizable Lando
 * artifact module. Validation errors during *loading* surface as a single
 * thrown error, not a `ValidationResult` — the caller can't proceed
 * without a parsed value.
 */
export class SchemaParseError extends Error {
  override readonly name = 'SchemaParseError';
  readonly code = ERROR_CODES.L_SCHEMA_PARSE;
  constructor(message: string) {
    super(message);
  }
}
