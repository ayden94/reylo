// The engine's type model (engine/src/types.rs), apart from the engine's host so that the run time
// and the modules a build writes can name it without Node's types: `src/execute.ts` reads a
// parameter's type from a module's `inputs`.
export type EngineType =
  /** `name` lists two types or more (`int8|float8`) where the value is one or the other. */
  | { kind: "scalar"; name: string }
  | { kind: "enum"; name: string; labels: string[] }
  /** `nullable_items` only where the statement builds the array; any other array is `T[]` (docs/driver-decoding.md, 2026-10-03). */
  | { kind: "array"; of: EngineType; nullable_items?: boolean }
  | { kind: "object"; fields: { name: string; type: EngineType; nullable: boolean }[] }
  | { kind: "composite"; name: string; fields: { name: string; type: EngineType; nullable: boolean }[] }
  | { kind: "list"; of: EngineType; nullable_items: boolean }
  /** Any JSON, or with `schema` the values a CHECK's JSON Schema admits, as the engine's canonical schema (engine/src/schema_type.rs). */
  | { kind: "json"; schema?: JsonSchema }
  | { kind: "unknown" };

/**
 * The engine's canonical JSON Schema of a JSON value: `true`, `false`, `{}`, `type` (one name or a
 * list), `enum`, `anyOf`, an array's `prefixItems`, `items` and `minItems`, and an object's
 * `properties`, `required` and `additionalProperties`, which it always states.
 */
export type JsonSchema = boolean | { readonly [key: string]: unknown };
