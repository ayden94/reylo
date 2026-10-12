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
  | { kind: "json" }
  | { kind: "unknown" };
